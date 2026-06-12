use crate::api::extractor::AuthUser;
use crate::api::models::profile::{
    CreateProfileRequest, ProfileRequestRecord, ProfileRequestResponse,
};
use crate::api::models::{ApiError, db_uuid};
use crate::api::{db, helpers};
use crate::{ApiContext, notifier};
use axum::Json;
use axum::extract::State;
use http::StatusCode;
use uuid::Uuid;

#[utoipa::path(
    post,
    path = "/profile-requests/",
    tag = "profile-request",
    request_body = CreateProfileRequest,
    responses(
        (status = 201, description = "Profile request created", body = ProfileRequestResponse),
        (status = 400, description = "Bad request", body = crate::api::models::ErrorResponse),
        (status = 401, description = "Unauthorized", body = crate::api::models::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::api::models::ErrorResponse),
    ),
    security(("bearer_auth" = []))
)]
pub async fn create_profile_request(
    State(state): State<ApiContext>,
    AuthUser(user): AuthUser,
    Json(payload): Json<CreateProfileRequest>,
) -> Result<(StatusCode, Json<ProfileRequestResponse>), ApiError> {
    let request = sqlx::query_as::<_, ProfileRequestRecord>(
        r#"
        insert into profile_request (
            profile_request_id,
            user_id,
            requested_profile_data_id,
            status
        )
        values ($1, $2, $3, 'sent')
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
    .bind(db_uuid(Uuid::now_v7()))
    .bind(db_uuid(user.user_id))
    .bind(db_uuid(payload.agent_data_id))
    .fetch_one(&state.db)
    .await?;

    notifier::notify_user(
        &state,
        user.user_id,
        "Profile request submitted. Review queue active. Gameplay access remains available while moderators verify dossier.",
    )
    .await;
    notifier::notify_admins(
        &state,
        format!(
            "New profile request {} waiting for moderation.",
            request.profile_request_id
        ),
    )
    .await;

    Ok((
        StatusCode::CREATED,
        Json(helpers::to_profile_request_response(request)),
    ))
}
