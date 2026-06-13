use crate::ApiContext;
use crate::db;
use crate::models::profile::{ProfileRequestRecord, ProfileRequestResponse, UpdateProfileRequest};
use crate::models::{ApiError, db_uuid};
use crate::rest::extractor::AuthUser;
use crate::rest::helpers;
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
    AuthUser(user): AuthUser,
    Path(request_id): Path<Uuid>,
    Json(payload): Json<UpdateProfileRequest>,
) -> Result<Json<ProfileRequestResponse>, ApiError> {
    let mut tx = state.db.begin().await?;
    let existing = sqlx::query_as::<_, ProfileRequestRecord>(
        r#"
        select
            cast(profile_request_id as text) as profile_request_id,
            cast(user_id as text) as user_id,
            cast(requested_profile_data_id as text) as requested_profile_data_id,
            status,
            reviewer_note,
            cast(reviewed_at as text) as reviewed_at,
            cast(created_at as text) as created_at,
            cast(updated_at as text) as updated_at
        from profile_request
        where profile_request_id = cast($1 as uuid)
        "#,
    )
    .bind(db_uuid(request_id))
    .fetch_optional(&mut *tx)
    .await?
    .ok_or(ApiError::NotFound)?;

    helpers::ensure_owner(&user, existing.user_id)?;
    if existing.status != "sent" {
        return Err(ApiError::Forbidden);
    }

    if let Some(profile_data) = payload.requested_profile_data.as_ref() {
        db::update_agent_data_from_profile(
            &state.db,
            existing.requested_profile_data_id,
            profile_data,
        )
        .await?;
    }

    let request = sqlx::query_as::<_, ProfileRequestRecord>(
        r#"
        update profile_request
        set updated_at = now()
        where profile_request_id = cast($1 as uuid)
        returning
            cast(profile_request_id as text) as profile_request_id,
            cast(user_id as text) as user_id,
            cast(requested_profile_data_id as text) as requested_profile_data_id,
            status,
            reviewer_note,
            cast(reviewed_at as text) as reviewed_at,
            cast(created_at as text) as created_at,
            cast(updated_at as text) as updated_at
        "#,
    )
    .bind(db_uuid(request_id))
    .fetch_optional(&mut *tx)
    .await?
    .ok_or(ApiError::NotFound)?;

    tx.commit().await?;

    Ok(Json(helpers::to_profile_request_response(request)))
}
