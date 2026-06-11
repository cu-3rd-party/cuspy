use crate::ApiContext;
use crate::api::extractor::AuthUser;
use crate::api::models::{ApiError, db_uuid};
use crate::api::{extractor, helpers};
use axum::extract::{Path, State};
use http::{HeaderMap, StatusCode};
use uuid::Uuid;

#[utoipa::path(
    delete,
    path = "/user/{user_id}",
    tag = "user",
    params(("user_id" = Uuid, Path, description = "User id")),
    responses(
        (status = 204, description = "User deleted"),
        (status = 403, description = "Forbidden", body = crate::api::models::ErrorResponse),
        (status = 404, description = "User not found", body = crate::api::models::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::api::models::ErrorResponse),
    ),
    security(("bearer_auth" = []))
)]
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
