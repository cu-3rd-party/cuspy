use crate::ApiContext;
use crate::models::user::{UpdateUserRequest, UserRecord, UserResponse};
use crate::models::{ApiError, db_uuid};
use crate::rest::extractor::AuthUser;
use crate::rest::helpers;
use axum::Json;
use axum::extract::{Path, State};
use uuid::Uuid;

#[utoipa::path(
    patch,
    path = "/user/{user_id}",
    tag = "user",
    params(("user_id" = Uuid, Path, description = "User id")),
    request_body = UpdateUserRequest,
    responses(
        (status = 200, description = "Updated user", body = UserResponse),
        (status = 403, description = "Forbidden", body = crate::models::ErrorResponse),
        (status = 404, description = "User not found", body = crate::models::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::models::ErrorResponse),
    ),
    security(("bearer_auth" = []))
)]
pub async fn update_user(
    State(state): State<ApiContext>,
    AuthUser(user): AuthUser,
    Path(user_id): Path<Uuid>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>, ApiError> {
    helpers::ensure_owner(&user, user_id)?;

    let user = sqlx::query_as::<_, UserRecord>(
        r#"
        update "user"
        set
            telegram_id = coalesce($2, telegram_id),
            agent_name = coalesce($3, agent_name)
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
    .fetch_optional(&state.db)
    .await?
    .ok_or(ApiError::NotFound)?;

    Ok(Json(helpers::to_user_response(user)))
}
