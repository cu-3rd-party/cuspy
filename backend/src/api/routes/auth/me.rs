use axum::extract::State;
use axum::Json;
use http::HeaderMap;
use crate::api::helpers;
use crate::api::models::ApiError;
use crate::api::models::user::UserResponse;
use crate::ApiContext;

pub async fn me(
    State(state): State<ApiContext>,
    headers: HeaderMap,
) -> Result<Json<UserResponse>, ApiError> {
    helpers::optional_telegram_user_id(&headers, &state)?;
    let auth = helpers::require_bearer_token(&headers, &state)?;
    let user = helpers::fetch_user(&state.db, auth.user_id).await?;
    Ok(Json(helpers::to_user_response(user)))
}
