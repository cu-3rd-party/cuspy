use crate::ApiContext;
use crate::api::db;
use crate::api::extractor::AuthUser;
use crate::api::helpers;
use crate::api::models::profile::{
    ProfileRequestRecord, ProfileRequestResponse, UpdateProfileRequest,
};
use crate::api::models::{ApiError, db_uuid};
use axum::Json;
use axum::extract::{Path, State};
use uuid::Uuid;

#[utoipa::path(
    put,
    path = "/profile-requests/{request_id}",
    tag = "profile-request",
    params(("request_id" = Uuid, Path, description = "Profile request id")),
    request_body = UpdateProfileRequest,
    responses(
        (status = 200, description = "Profile request updated", body = ProfileRequestResponse),
        (status = 403, description = "Forbidden", body = crate::api::models::ErrorResponse),
        (status = 404, description = "Profile request not found", body = crate::api::models::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::api::models::ErrorResponse),
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
            profile_request_id,
            user_id,
            requested_profile_data_id,
            status,
            reviewer_note,
            reviewed_at,
            created_at,
            updated_at
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
            profile_request_id,
            user_id,
            requested_profile_data_id,
            status,
            reviewer_note,
            reviewed_at,
            created_at,
            updated_at
        "#,
    )
    .bind(db_uuid(request_id))
    .fetch_optional(&mut *tx)
    .await?
    .ok_or(ApiError::NotFound)?;

    Ok(Json(helpers::to_profile_request_response(request)))
}
