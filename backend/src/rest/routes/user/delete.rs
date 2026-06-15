use crate::ApiContext;
use crate::models::ApiError;
use crate::models::user::User;
use crate::rest::extractor::AuthUser;
use crate::rest::helpers;
use axum::extract::{Path, State};
use http::StatusCode;
use uuid::Uuid;

pub(crate) async fn delete_user_record(db: &sqlx::PgPool, user_id: Uuid) -> Result<(), ApiError> {
    if !User::delete(db, user_id).await? {
        return Err(ApiError::NotFound);
    }

    Ok(())
}

#[utoipa::path(
    delete,
    path = "/api/user/{user_id}",
    tag = "user",
    params(("user_id" = Uuid, Path, description = "User id")),
    responses(
        (status = 204, description = "User deleted"),
        (status = 403, description = "Forbidden", body = crate::models::ErrorResponse),
        (status = 404, description = "User not found", body = crate::models::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::models::ErrorResponse),
    ),
    security(("bearer_auth" = []))
)]
pub async fn delete_user(
    State(state): State<ApiContext>,
    AuthUser(user): AuthUser,
    Path(user_id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    helpers::ensure_owner(&user, user_id)?;
    delete_user_record(&state.db, user_id).await?;

    Ok(StatusCode::NO_CONTENT)
}
