use crate::ApiContext;
use crate::api::extractor::AuthUser;
use crate::api::models::{ApiError, db_uuid};
use crate::api::{extractor, helpers};
use axum::extract::{Path, State};
use http::{HeaderMap, StatusCode};
use uuid::Uuid;

pub async fn delete_profile_request(
    State(state): State<ApiContext>,
    AuthUser(user): AuthUser,
    Path(request_id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    let owner_user_id = sqlx::query_scalar::<_, String>(r#"select cast(user_id as text) from profile_request where profile_request_id = cast($1 as uuid)"#)
    .bind(db_uuid(request_id))
    .fetch_optional(&state.db)
    .await?
    .ok_or(ApiError::NotFound)?;
    let owner_user_id = Uuid::parse_str(&owner_user_id)
        .map_err(|error| ApiError::Database(sqlx::Error::Decode(Box::new(error))))?;
    helpers::ensure_owner(&user, owner_user_id)?;

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
