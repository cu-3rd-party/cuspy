use crate::ApiContext;
use crate::models::ApiError;
use crate::models::profile::ProfileRequestRecord;
use crate::rest::extractor::AuthUser;
use axum::extract::{Path, State};
use http::StatusCode;
use uuid::Uuid;

#[utoipa::path(
    delete,
    path = "/api/profile-requests/{request_id}",
    tag = "profile-request",
    params(("request_id" = Uuid, Path, description = "Profile request id")),
    responses(
        (status = 204, description = "Profile request deleted"),
        (status = 304, description = "Profile request wasn't deleted (for some reason db returned 0 rows changed)"),
        (status = 403, description = "Forbidden", body = crate::models::ErrorResponse),
        (status = 404, description = "Profile request not found", body = crate::models::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::models::ErrorResponse),
    ),
    security(("bearer_auth" = []))
)]
pub async fn delete_profile_request(
    State(state): State<ApiContext>,
    AuthUser(user): AuthUser,
    Path(request_id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    let mut tx = state.db.begin().await?;
    let profile = ProfileRequestRecord::get_by_id(&mut *tx, request_id)
        .await
        .ok_or(ApiError::NotFound)?;
    if profile.user_id != user.user_id && !user.is_admin {
        return Err(ApiError::Forbidden);
    }
    let deleted = profile.delete(&mut *tx).await?;
    if !deleted {
        return Ok(StatusCode::NOT_MODIFIED);
    }
    tx.commit().await?; // удаляем только если бд реально чето удалила

    Ok(StatusCode::NO_CONTENT)
}
