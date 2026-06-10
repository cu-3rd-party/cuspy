use axum::extract::{Path, State};
use http::{HeaderMap, StatusCode};
use uuid::Uuid;
use crate::api::helpers;
use crate::api::models::{db_uuid, ApiError};
use crate::ApiContext;

pub async fn delete_user(
    State(state): State<ApiContext>,
    headers: HeaderMap,
    Path(user_id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    helpers::optional_telegram_user_id(&headers, &state)?;
    let auth = helpers::require_bearer_token(&headers, &state)?;
    helpers::ensure_owner(&auth, user_id)?;

    let result = sqlx::query(r#"delete from "user" where user_id = cast($1 as uuid)"#)
        .bind(db_uuid(user_id))
        .execute(&state.db)
        .await?;

    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound);
    }

    Ok(StatusCode::NO_CONTENT)
}