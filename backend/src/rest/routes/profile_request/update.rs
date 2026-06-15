use crate::ApiContext;
use crate::models::profile::{ProfileRequestRecord, ProfileRequestResponse, UpdateProfileRequest};
use crate::models::{ApiError};
use crate::rest::extractor::{AdminUser};
use axum::Json;
use axum::extract::{Path, State};
use uuid::Uuid;

#[utoipa::path(
    put,
    path = "/api/profile-requests/{request_id}",
    tag = "profile-request",
    params(("request_id" = Uuid, Path, description = "Profile request id")),
    request_body = UpdateProfileRequest,
    responses(
        (status = 200, description = "Profile request updated", body = ProfileRequestResponse),
        (status = 403, description = "Forbidden", body = crate::models::ErrorResponse),
        (status = 404, description = "Profile request not found", body = crate::models::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::models::ErrorResponse),
    ),
    security(("bearer_auth" = []))
)]
pub async fn update_profile_request(
    State(state): State<ApiContext>,
    AdminUser(_user): AdminUser,
    Path(request_id): Path<Uuid>,
    Json(req): Json<UpdateProfileRequest>,
) -> Result<Json<ProfileRequestResponse>, ApiError> {
    let mut tx = state.db.begin().await?;
    let profile = ProfileRequestRecord::get_by_id(&mut *tx, request_id).await.ok_or(ApiError::NotFound)?;

    let profile = profile
        .update(&mut *tx, req.status, req.reviewer_note)
        .await?;

    let response = profile.into_response(&mut *tx).await?;
    tx.commit().await?;

    Ok(Json(response))
}
