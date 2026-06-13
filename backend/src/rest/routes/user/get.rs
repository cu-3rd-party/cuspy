use crate::ApiContext;
use crate::rest::extractor::AuthUser;
use crate::rest::helpers;
use crate::rest::models::ApiError;
use crate::rest::models::user::UserResponse;
use axum::Json;
use axum::extract::{Path, State};
use uuid::Uuid;

#[utoipa::path(
    get,
    path = "/user/{user_id}",
    tag = "user",
    params(("user_id" = Uuid, Path, description = "User id")),
    responses(
        (status = 200, description = "User details", body = UserResponse),
        (status = 403, description = "Forbidden", body = crate::rest::models::ErrorResponse),
        (status = 404, description = "User not found", body = crate::rest::models::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::rest::models::ErrorResponse),
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_user(
    State(state): State<ApiContext>,
    AuthUser(user): AuthUser,
    Path(user_id): Path<Uuid>,
) -> Result<Json<UserResponse>, ApiError> {
    helpers::ensure_owner(&user, user_id)?;
    let user = helpers::fetch_user(&state.db, user_id).await?;
    Ok(Json(helpers::to_user_response(user)))
}
