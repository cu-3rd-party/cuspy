use crate::ApiContext;
use crate::rest::extractor::AuthUser;
use crate::rest::helpers;
use crate::rest::models::ApiError;
use crate::rest::models::user::UserResponse;
use axum::Json;
use axum::extract::State;

#[utoipa::path(
    get,
    path = "/auth/me",
    tag = "auth",
    responses(
        (status = 200, description = "Current authenticated user", body = UserResponse),
        (status = 401, description = "Unauthorized", body = crate::rest::models::ErrorResponse),
        (status = 404, description = "User not found", body = crate::rest::models::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::rest::models::ErrorResponse),
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
