use crate::ApiContext;
use crate::models::user::{CreateUserRequest, UpdateUserRequest, User, UserResponse};
use crate::models::{ApiError, db_uuid};
use crate::rest::extractor::AdminUser;
use crate::rest::helpers;
use axum::extract::{Path, State};
use axum::routing::get;
use axum::{Json, Router};
use http::StatusCode;
use uuid::Uuid;

async fn list_user_responses(db: sqlx::PgPool) -> Result<Vec<UserResponse>, ApiError> {
    let users = User::list(&db).await?;
    let mut response = Vec::with_capacity(users.len());
    for user in users {
        response.push(user.into_response(&db).await?);
    }
    Ok(response)
}

async fn create_user_response(
    db: sqlx::PgPool,
    payload: CreateUserRequest,
) -> Result<UserResponse, ApiError> {
    let mut tx = db.begin().await?;
    let user = User::create(&mut *tx, payload.username, payload.is_admin.unwrap_or(false), None)
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
    user.into_response(&db).await
}

async fn get_user_response(db: sqlx::PgPool, user_id: Uuid) -> Result<UserResponse, ApiError> {
    let user = User::get_by_id(&db, user_id).await.ok_or(ApiError::NotFound)?;
    user.into_response(&db).await
}

async fn update_user_response(
    db: sqlx::PgPool,
    user_id: Uuid,
    payload: UpdateUserRequest,
) -> Result<UserResponse, ApiError> {
    let mut user = User::get_by_id(&db, user_id).await.ok_or(ApiError::NotFound)?;
    if let Some(username) = payload.username {
        user.username = Some(username);
    }
    if let Some(agent_data_id) = payload.agent_data_id {
        user.agent_data_id = Some(agent_data_id);
    }
    if let Some(is_admin) = payload.is_admin {
        user.is_admin = is_admin;
    }

    user.update(&db).await?;
    user.into_response(&db).await
}

async fn delete_user_record(db: sqlx::PgPool, user_id: Uuid) -> Result<(), ApiError> {
    if !User::delete(&db, user_id).await? {
        return Err(ApiError::NotFound);
    }

    Ok(())
}

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
    path = "/api/admin/user/",
    tag = "admin",
    responses(
        (status = 200, description = "List users", body = [UserResponse]),
        (status = 401, description = "Unauthorized", body = crate::models::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::models::ErrorResponse),
    ),
    security(("bearer_auth" = []))
)]
pub async fn admin_list_users(
    State(state): State<ApiContext>,
    AdminUser(_user): AdminUser,
) -> Result<Json<Vec<UserResponse>>, ApiError> {
    Ok(Json(list_user_responses(state.db.clone()).await?))
}

#[utoipa::path(
    post,
    path = "/api/admin/user/",
    tag = "admin",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User created", body = UserResponse),
        (status = 400, description = "Bad request", body = crate::models::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::models::ErrorResponse),
    ),
    security(("bearer_auth" = []))
)]
pub async fn admin_create_user(
    State(state): State<ApiContext>,
    AdminUser(_user): AdminUser,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<UserResponse>), ApiError> {
    Ok((StatusCode::CREATED, Json(create_user_response(state.db.clone(), payload).await?)))
}

#[utoipa::path(
    get,
    path = "/api/admin/user/{user_id}",
    tag = "admin",
    params(("user_id" = Uuid, Path, description = "User id")),
    responses(
        (status = 200, description = "User details", body = UserResponse),
        (status = 404, description = "User not found", body = crate::models::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::models::ErrorResponse),
    ),
    security(("bearer_auth" = []))
)]
pub async fn admin_get_user(
    State(state): State<ApiContext>,
    AdminUser(_user): AdminUser,
    Path(user_id): Path<Uuid>,
) -> Result<Json<UserResponse>, ApiError> {
    Ok(Json(get_user_response(state.db.clone(), user_id).await?))
}

#[utoipa::path(
    patch,
    path = "/api/admin/user/{user_id}",
    tag = "admin",
    params(("user_id" = Uuid, Path, description = "User id")),
    request_body = UpdateUserRequest,
    responses(
        (status = 200, description = "Updated user", body = UserResponse),
        (status = 404, description = "User not found", body = crate::models::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::models::ErrorResponse),
    ),
    security(("bearer_auth" = []))
)]
pub async fn admin_update_user(
    State(state): State<ApiContext>,
    AdminUser(_user): AdminUser,
    Path(user_id): Path<Uuid>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>, ApiError> {
    Ok(Json(update_user_response(state.db.clone(), user_id, payload).await?))
}

#[utoipa::path(
    delete,
    path = "/api/admin/user/{user_id}",
    tag = "admin",
    params(("user_id" = Uuid, Path, description = "User id")),
    responses(
        (status = 204, description = "User deleted"),
        (status = 404, description = "User not found", body = crate::models::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::models::ErrorResponse),
    ),
    security(("bearer_auth" = []))
)]
pub async fn admin_delete_user(
    State(state): State<ApiContext>,
    AdminUser(_user): AdminUser,
    Path(user_id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    delete_user_record(state.db.clone(), user_id).await?;

    Ok(StatusCode::NO_CONTENT)
}
