use axum::extract::{Path, State};
use http::HeaderMap;
use uuid::Uuid;
use axum::Json;
use crate::api::helpers;
use crate::api::models::ApiError;
use crate::api::models::user::UserResponse;
use crate::ApiContext;

pub async fn get_user(
    State(state): State<ApiContext>,
    headers: HeaderMap,
    Path(user_id): Path<Uuid>,
) -> Result<Json<UserResponse>, ApiError> {
    helpers::optional_telegram_user_id(&headers, &state)?;
    let auth = helpers::require_bearer_token(&headers, &state)?;
    helpers::ensure_owner(&auth, user_id)?;
    let user = helpers::fetch_user(&state.db, user_id).await?;
    Ok(Json(helpers::to_user_response(user)))
}