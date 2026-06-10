use crate::ApiContext;
use crate::api::helpers;
use crate::api::models::user::{CreateUserRequest, UpdateUserRequest, UserRecord, UserResponse};
use crate::api::models::{ApiError, db_uuid};
use axum::Json;
use axum::extract::{Path, State};
use http::{HeaderMap, StatusCode};
use uuid::Uuid;

pub async fn admin_list_users(
    State(state): State<ApiContext>,
    headers: HeaderMap,
) -> Result<Json<Vec<UserResponse>>, ApiError> {
    helpers::require_admin(&headers, &state)?;
    let users = sqlx::query_as::<_, UserRecord>(
        r#"
        select
            cast(user_id as text) as user_id,
            telegram_id,
            agent_name,
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

pub async fn admin_create_user(
    State(state): State<ApiContext>,
    headers: HeaderMap,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<UserResponse>), ApiError> {
    helpers::require_admin(&headers, &state)?;
    let user = sqlx::query_as::<_, UserRecord>(
        r#"
        insert into "user" (user_id, telegram_id, agent_name, is_admin)
        values (cast($1 as uuid), $2, $3, $4)
        returning
            cast(user_id as text) as user_id,
            telegram_id,
            agent_name,
            is_admin,
            cast(created_at as text) as created_at,
            cast(updated_at as text) as updated_at
        "#,
    )
    .bind(db_uuid(Uuid::now_v7()))
    .bind(payload.telegram_id)
    .bind(payload.agent_name)
    .bind(payload.is_admin.unwrap_or(false))
    .fetch_one(&state.db)
    .await?;

    Ok((StatusCode::CREATED, Json(helpers::to_user_response(user))))
}

pub async fn admin_get_user(
    State(state): State<ApiContext>,
    headers: HeaderMap,
    Path(user_id): Path<Uuid>,
) -> Result<Json<UserResponse>, ApiError> {
    helpers::require_admin(&headers, &state)?;
    let user = helpers::fetch_user(&state.db, user_id).await?;
    Ok(Json(helpers::to_user_response(user)))
}

pub async fn admin_update_user(
    State(state): State<ApiContext>,
    headers: HeaderMap,
    Path(user_id): Path<Uuid>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>, ApiError> {
    helpers::require_admin(&headers, &state)?;

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

pub async fn admin_delete_user(
    State(state): State<ApiContext>,
    headers: HeaderMap,
    Path(user_id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    helpers::require_admin(&headers, &state)?;
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
