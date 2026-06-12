use crate::ApiContext;
use crate::api::extractor::AuthUser;
use crate::api::models::ApiError;
use crate::api::models::user::UserResponse;
use crate::api::helpers;
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
        (status = 403, description = "Forbidden", body = crate::api::models::ErrorResponse),
        (status = 404, description = "User not found", body = crate::api::models::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::api::models::ErrorResponse),
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
