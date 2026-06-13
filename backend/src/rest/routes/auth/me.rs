use crate::ApiContext;
use crate::models::ApiError;
use crate::models::user::UserResponse;
use crate::rest::extractor::AuthUser;
use crate::rest::helpers;
use axum::Json;
use axum::extract::State;

#[utoipa::path(
    get,
    path = "/api/auth/me",
    tag = "auth",
    responses(
        (status = 200, description = "Current authenticated user", body = UserResponse),
        (status = 401, description = "Unauthorized", body = crate::models::ErrorResponse),
        (status = 404, description = "User not found", body = crate::models::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::models::ErrorResponse),
    ),
    security(("bearer_auth" = []))
)]
pub async fn me(
    State(state): State<ApiContext>,
    AuthUser(user): AuthUser,
) -> Result<Json<UserResponse>, ApiError> {
    let user = helpers::fetch_user(&state.db, user.user_id).await?;
    Ok(Json(helpers::to_user_response(user)))
}
