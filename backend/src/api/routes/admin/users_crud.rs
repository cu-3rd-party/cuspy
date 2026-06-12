use crate::ApiContext;
use crate::api::extractor::AdminUser;
use crate::api::helpers;
use crate::api::models::user::{CreateUserRequest, UpdateUserRequest, UserRecord, UserResponse};
use crate::api::models::{ApiError, db_uuid};
use axum::extract::{Path, State};
use axum::routing::get;
use axum::{Json, Router};
use http::StatusCode;
use uuid::Uuid;

pub fn users_router() -> Router<ApiContext> {
    Router::new()
        .route("/", get(admin_list_users).post(admin_create_user))
        .route(
            "/{user_id}",
            get(admin_get_user)
                .patch(admin_update_user)
                .delete(admin_delete_user),
        )
}

#[utoipa::path(
    get,
    path = "/admin/user/",
    tag = "admin",
    responses(
        (status = 200, description = "List users", body = [UserResponse]),
        (status = 401, description = "Unauthorized", body = crate::api::models::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::api::models::ErrorResponse),
    ),
    security(("bearer_auth" = []))
)]
pub async fn admin_list_users(
    State(state): State<ApiContext>,
    AdminUser(_user): AdminUser,
) -> Result<Json<Vec<UserResponse>>, ApiError> {
    let users = sqlx::query_as::<_, UserRecord>(
        r#"
        select
            cast(user_id as text) as user_id,
            telegram_id,
            agent_name,
            cast(agent_data_id as text) as agent_data_id,
            rating,
            is_admin,
            cast(created_at as text) as created_at,
            cast(updated_at as text) as updated_at
        from "user"
        order by created_at desc
        "#,
    )
    .fetch_all(&state.db)
    .await?;

    Ok(Json(users.iter().map(|u| u.into()).collect()))
}

#[utoipa::path(
    post,
    path = "/admin/user/",
    tag = "admin",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User created", body = UserResponse),
        (status = 400, description = "Bad request", body = crate::api::models::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::api::models::ErrorResponse),
    ),
    security(("bearer_auth" = []))
)]
pub async fn admin_create_user(
    State(state): State<ApiContext>,
    AdminUser(_user): AdminUser,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<UserResponse>), ApiError> {
    let mut tx = state.db.begin().await?;
    let user_id = Uuid::now_v7();

    let user = sqlx::query_as::<_, UserRecord>(
        r#"
        insert into "user" (user_id, telegram_id, agent_name, is_admin)
        values (cast($1 as uuid), $2, $3, $4)
        returning
            cast(user_id as text) as user_id,
            telegram_id,
            agent_name,
            cast(agent_data_id as text) as agent_data_id,
            rating,
            is_admin,
            cast(created_at as text) as created_at,
            cast(updated_at as text) as updated_at
        "#,
    )
    .bind(db_uuid(user_id))
    .bind(payload.telegram_id)
    .bind(payload.agent_name)
    .bind(payload.is_admin.unwrap_or(false))
    .fetch_one(&mut *tx)
    .await?;

    sqlx::query(
        r#"
        insert into rating_history (rating_history_id, user_id, rating, change, reason)
        values (cast($1 as uuid), cast($2 as uuid), $3, $4, $5)
        "#,
    )
    .bind(db_uuid(Uuid::now_v7()))
    .bind(db_uuid(user.user_id))
    .bind(helpers::DEFAULT_RATING)
    .bind(helpers::DEFAULT_RATING)
    .bind("initial_rating")
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    let user = helpers::fetch_user(&state.db, user.user_id).await?;

    Ok((StatusCode::CREATED, Json(helpers::to_user_response(user))))
}

#[utoipa::path(
    get,
    path = "/admin/user/{user_id}",
    tag = "admin",
    params(("user_id" = Uuid, Path, description = "User id")),
    responses(
        (status = 200, description = "User details", body = UserResponse),
        (status = 404, description = "User not found", body = crate::api::models::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::api::models::ErrorResponse),
    ),
    security(("bearer_auth" = []))
)]
pub async fn admin_get_user(
    State(state): State<ApiContext>,
    AdminUser(_user): AdminUser,
    Path(user_id): Path<Uuid>,
) -> Result<Json<UserResponse>, ApiError> {
    let user = helpers::fetch_user(&state.db, user_id).await?;
    Ok(Json(helpers::to_user_response(user)))
}

#[utoipa::path(
    patch,
    path = "/admin/user/{user_id}",
    tag = "admin",
    params(("user_id" = Uuid, Path, description = "User id")),
    request_body = UpdateUserRequest,
    responses(
        (status = 200, description = "Updated user", body = UserResponse),
        (status = 404, description = "User not found", body = crate::api::models::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::api::models::ErrorResponse),
    ),
    security(("bearer_auth" = []))
)]
pub async fn admin_update_user(
    State(state): State<ApiContext>,
    AdminUser(_user): AdminUser,
    Path(user_id): Path<Uuid>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>, ApiError> {
    let user = sqlx::query_as::<_, UserRecord>(
        r#"
        update "user"
        set
            telegram_id = coalesce($2, telegram_id),
            agent_name = coalesce($3, agent_name),
            is_admin = coalesce($4, is_admin)
        where user_id = cast($1 as uuid)
        returning
            cast(user_id as text) as user_id,
            telegram_id,
            agent_name,
            cast(agent_data_id as text) as agent_data_id,
            rating,
            is_admin,
            cast(created_at as text) as created_at,
            cast(updated_at as text) as updated_at
        "#,
    )
    .bind(db_uuid(user_id))
    .bind(payload.telegram_id)
    .bind(payload.agent_name)
    .bind(payload.is_admin)
    .fetch_optional(&state.db)
    .await?
    .ok_or(ApiError::NotFound)?;

    Ok(Json(helpers::to_user_response(user)))
}

#[utoipa::path(
    delete,
    path = "/admin/user/{user_id}",
    tag = "admin",
    params(("user_id" = Uuid, Path, description = "User id")),
    responses(
        (status = 204, description = "User deleted"),
        (status = 404, description = "User not found", body = crate::api::models::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::api::models::ErrorResponse),
    ),
    security(("bearer_auth" = []))
)]
pub async fn admin_delete_user(
    State(state): State<ApiContext>,
    AdminUser(_user): AdminUser,
    Path(user_id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    let result = sqlx::query(
        r#"
        delete from "user"
        where user_id = cast($1 as uuid)
        "#,
    )
    .bind(db_uuid(user_id))
    .execute(&state.db)
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound);
    }

    Ok(StatusCode::NO_CONTENT)
}
