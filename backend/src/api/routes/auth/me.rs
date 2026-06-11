use crate::ApiContext;
use crate::api::extractor::{AuthUser, User};
use crate::api::helpers;
use crate::api::models::ApiError;
use crate::api::models::user::UserResponse;
use axum::Json;
use axum::extract::State;

pub async fn me(
    State(state): State<ApiContext>,
    AuthUser(user): AuthUser,
) -> Result<Json<UserResponse>, ApiError> {
    let user = helpers::fetch_user(&state.db, user.user_id).await?;
    Ok(Json(helpers::to_user_response(user)))
}
