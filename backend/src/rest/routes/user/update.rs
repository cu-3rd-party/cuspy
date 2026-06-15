use crate::ApiContext;
use crate::models::ApiError;
use crate::models::user::{UpdateUserRequest, User, UserResponse};
use crate::rest::extractor::AuthUser;
use crate::rest::helpers;
use axum::Json;
use axum::extract::{Path, State};
use uuid::Uuid;

#[utoipa::path(
    patch,
    path = "/api/user/{user_id}",
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

    let mut user = User::get_by_id(&state.db, user_id)
        .await
        .ok_or(ApiError::NotFound)?;
    if let Some(username) = payload.username {
        user.username = Some(username);
    }
    if let Some(agent_data_id) = payload.agent_data_id {
        user.agent_data_id = Some(agent_data_id);
    }

    user.update(&state.db).await?;
    Ok(Json(user.into_response(&state.db).await?))
}
