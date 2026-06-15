use crate::ApiContext;
use crate::models::ApiError;
use crate::models::user::{User, UserResponse};
use crate::rest::extractor::AuthUser;
use axum::Json;
use axum::extract::{Path, State};
use uuid::Uuid;

#[utoipa::path(
    get,
    path = "/api/user/{user_id}",
    tag = "user",
    params(("user_id" = Uuid, Path, description = "User id")),
    responses(
        (status = 200, description = "User details", body = UserResponse),
        (status = 403, description = "Forbidden", body = crate::models::ErrorResponse),
        (status = 404, description = "User not found", body = crate::models::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::models::ErrorResponse),
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_user(
    State(state): State<ApiContext>,
    AuthUser(_user): AuthUser,
    Path(user_id): Path<Uuid>,
) -> Result<Json<UserResponse>, ApiError> {
    let user = User::get_by_id(&state.db, user_id).await.ok_or(ApiError::NotFound)?;
    Ok(Json(user.into_response(&state.db).await?))
}
