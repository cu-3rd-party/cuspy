use crate::ApiContext;
use crate::models::ApiError;
use crate::models::profile::{ProfileRequestRecord, ProfileRequestResponse};
use crate::rest::extractor::AuthUser;
use axum::Json;
use axum::extract::{Path, State};
use uuid::Uuid;

#[utoipa::path(
    get,
    path = "/api/profile-requests/{request_id}",
    tag = "profile-request",
    params(("request_id" = Uuid, Path, description = "Profile request id")),
    responses(
        (status = 200, description = "Profile request", body = ProfileRequestResponse),
        (status = 403, description = "Forbidden", body = crate::models::ErrorResponse),
        (status = 404, description = "Profile request not found", body = crate::models::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::models::ErrorResponse),
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_profile_request(
    State(state): State<ApiContext>,
    AuthUser(user): AuthUser,
    Path(request_id): Path<Uuid>,
) -> Result<Json<ProfileRequestResponse>, ApiError> {
    let mut tx = state.db.begin().await?;
    let profile = ProfileRequestRecord::get_by_id(&mut *tx, request_id)
        .await
        .ok_or(ApiError::NotFound)?;
    if profile.user_id != user.user_id && !user.is_admin {
        return Err(ApiError::Forbidden);
    }
    Ok(Json(profile.into_response(&mut *tx).await?))
}
