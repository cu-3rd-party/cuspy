use crate::ApiContext;
use crate::api::models::ApiError;
use crate::api::models::user::UserResponse;
use crate::api::{extractor, helpers};
use axum::Json;
use axum::extract::{Path, State};
use http::HeaderMap;
use uuid::Uuid;
use crate::api::extractor::AuthUser;

pub async fn get_user(
    State(state): State<ApiContext>,
    AuthUser(user): AuthUser,
    Path(user_id): Path<Uuid>,
) -> Result<Json<UserResponse>, ApiError> {
    helpers::ensure_owner(&user, user_id)?;
    let user = helpers::fetch_user(&state.db, user_id).await?;
    Ok(Json(helpers::to_user_response(user)))
}
