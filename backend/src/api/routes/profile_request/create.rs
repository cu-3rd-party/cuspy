use crate::api::models::profile::{
    CreateProfileRequest, ProfileRequestRecord, ProfileRequestResponse,
};
use crate::api::models::{db_uuid, ApiError};
use crate::api::{extractor, helpers};
use crate::{notifier, ApiContext};
use axum::Json;
use axum::extract::State;
use http::{HeaderMap, StatusCode};
use uuid::Uuid;
use crate::api::extractor::AuthUser;

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
    .bind("") // TODO:
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
