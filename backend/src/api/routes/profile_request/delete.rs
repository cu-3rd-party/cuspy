use axum::extract::{Path, State};
use http::{HeaderMap, StatusCode};
use uuid::Uuid;
use crate::api::helpers;
use crate::api::models::{db_uuid, ApiError};
use crate::ApiContext;

pub async fn delete_profile_request(
    State(state): State<ApiContext>,
    headers: HeaderMap,
    Path(request_id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    helpers::optional_telegram_user_id(&headers, &state)?;
    let auth = helpers::require_bearer_token(&headers, &state)?;
    let owner_user_id = sqlx::query_scalar::<_, String>(r#"select cast(user_id as text) from profile_request where profile_request_id = cast($1 as uuid)"#)
    .bind(db_uuid(request_id))
    .fetch_optional(&state.db)
    .await?
    .ok_or(ApiError::NotFound)?;
    let owner_user_id = Uuid::parse_str(&owner_user_id)
        .map_err(|error| ApiError::Database(sqlx::Error::Decode(Box::new(error))))?;
    helpers::ensure_owner(&auth, owner_user_id)?;

    let result =
        sqlx::query(r#"delete from profile_request where profile_request_id = cast($1 as uuid)"#)
            .bind(db_uuid(request_id))
            .execute(&state.db)
            .await?;

    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound);
    }

    Ok(StatusCode::NO_CONTENT)
}