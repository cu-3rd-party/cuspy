use crate::ApiContext;
use crate::api::models::{db_uuid, ApiError};
use crate::api::{extractor, helpers};
use axum::extract::{Path, State};
use http::{HeaderMap, StatusCode};
use uuid::Uuid;
use crate::api::extractor::AuthUser;

pub async fn delete_user(
    State(state): State<ApiContext>,
    AuthUser(user): AuthUser,
    Path(user_id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    helpers::ensure_owner(&user, user_id)?;

    let result = sqlx::query(r#"delete from "user" where user_id = cast($1 as uuid)"#)
        .bind(db_uuid(user_id))
        .execute(&state.db)
        .await?;

    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound);
    }

    Ok(StatusCode::NO_CONTENT)
}
