use crate::AppState;
use crate::api::helpers;
use crate::api::models::{db_json, db_uuid};
use crate::api::models::ApiError;
use crate::api::models::profile::{
    CreateProfileCreationRequest, ProfileCreationRequestRecord, ProfileCreationRequestResponse,
    UpdateProfileCreationRequest,
};
use crate::notifier;
use axum::Json;
use axum::extract::{Path, State};
use http::{HeaderMap, StatusCode};
use uuid::Uuid;

pub async fn list_profile_creation_requests(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Vec<ProfileCreationRequestResponse>>, ApiError> {
    helpers::optional_telegram_user_id(&headers, &state)?;
    let auth = helpers::require_bearer_token(&headers, &state)?;
    let requests = sqlx::query_as::<_, ProfileCreationRequestRecord>(
        r#"
        select
            profile_creation_request_id,
            user_id,
            requested_profile_data,
            status,
            reviewer_note,
            reviewed_at,
            created_at,
            updated_at
        from profile_creation_request
        where user_id = $1
        order by created_at desc
        "#,
    )
    .bind(db_uuid(auth.user_id))
    .fetch_all(&state.db)
    .await?;

    Ok(Json(
        requests
            .into_iter()
            .map(helpers::to_profile_creation_request_response)
            .collect(),
    ))
}

pub async fn create_profile_creation_request(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<CreateProfileCreationRequest>,
) -> Result<(StatusCode, Json<ProfileCreationRequestResponse>), ApiError> {
    helpers::optional_telegram_user_id(&headers, &state)?;
    let auth = helpers::require_bearer_token(&headers, &state)?;
    let requested_profile_data =
        helpers::normalize_profile_data(Some(payload.requested_profile_data))?;
    let request = sqlx::query_as::<_, ProfileCreationRequestRecord>(
        r#"
        insert into profile_creation_request (
            profile_creation_request_id,
            user_id,
            requested_profile_data,
            status
        )
        values ($1, $2, $3, 'sent')
        returning
            profile_creation_request_id,
            user_id,
            requested_profile_data,
            status,
            reviewer_note,
            reviewed_at,
            created_at,
            updated_at
        "#,
    )
    .bind(db_uuid(Uuid::now_v7()))
    .bind(db_uuid(auth.user_id))
    .bind(db_json(&requested_profile_data))
    .fetch_one(&state.db)
    .await?;

    notifier::notify_user(
        &state,
        auth.user_id,
        "Profile request submitted. Review queue active. Gameplay access remains available while moderators verify dossier.",
    )
    .await;
    notifier::notify_admins(
        &state,
        format!(
            "New profile request {} waiting for moderation.",
            request.profile_creation_request_id
        ),
    )
    .await;

    Ok((
        StatusCode::CREATED,
        Json(helpers::to_profile_creation_request_response(request)),
    ))
}

pub async fn get_profile_creation_request(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(request_id): Path<Uuid>,
) -> Result<Json<ProfileCreationRequestResponse>, ApiError> {
    helpers::optional_telegram_user_id(&headers, &state)?;
    let auth = helpers::require_bearer_token(&headers, &state)?;
    let request = sqlx::query_as::<_, ProfileCreationRequestRecord>(
        r#"
        select
            profile_creation_request_id,
            user_id,
            requested_profile_data,
            status,
            reviewer_note,
            reviewed_at,
            created_at,
            updated_at
        from profile_creation_request
        where profile_creation_request_id = $1
        "#,
    )
    .bind(db_uuid(request_id))
    .fetch_optional(&state.db)
    .await?
    .ok_or(ApiError::NotFound)?;

    helpers::ensure_owner(&auth, request.user_id)?;
    Ok(Json(helpers::to_profile_creation_request_response(request)))
}

pub async fn update_profile_creation_request(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(request_id): Path<Uuid>,
    Json(payload): Json<UpdateProfileCreationRequest>,
) -> Result<Json<ProfileCreationRequestResponse>, ApiError> {
    helpers::optional_telegram_user_id(&headers, &state)?;
    let auth = helpers::require_bearer_token(&headers, &state)?;
    let existing = sqlx::query_as::<_, ProfileCreationRequestRecord>(
        r#"
        select
            profile_creation_request_id,
            user_id,
            requested_profile_data,
            status,
            reviewer_note,
            reviewed_at,
            created_at,
            updated_at
        from profile_creation_request
        where profile_creation_request_id = $1
        "#,
    )
    .bind(db_uuid(request_id))
    .fetch_optional(&state.db)
    .await?
    .ok_or(ApiError::NotFound)?;

    helpers::ensure_owner(&auth, existing.user_id)?;
    if existing.status != "sent" {
        return Err(ApiError::Forbidden);
    }

    let requested_profile_data = match payload.requested_profile_data {
        Some(value) => Some(helpers::normalize_profile_data(Some(value))?),
        None => None,
    };

    let request = sqlx::query_as::<_, ProfileCreationRequestRecord>(
        r#"
        update profile_creation_request
        set requested_profile_data = coalesce($2, requested_profile_data)
        where profile_creation_request_id = $1
        returning
            profile_creation_request_id,
            user_id,
            requested_profile_data,
            status,
            reviewer_note,
            reviewed_at,
            created_at,
            updated_at
        "#,
    )
    .bind(db_uuid(request_id))
    .bind(requested_profile_data.as_ref().map(db_json))
    .fetch_optional(&state.db)
    .await?
    .ok_or(ApiError::NotFound)?;

    Ok(Json(helpers::to_profile_creation_request_response(request)))
}

pub async fn delete_profile_creation_request(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(request_id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    helpers::optional_telegram_user_id(&headers, &state)?;
    let auth = helpers::require_bearer_token(&headers, &state)?;
    let owner_user_id = sqlx::query_scalar::<_, String>(
        "select user_id from profile_creation_request where profile_creation_request_id = $1",
    )
    .bind(db_uuid(request_id))
    .fetch_optional(&state.db)
    .await?
    .ok_or(ApiError::NotFound)?;
    let owner_user_id = Uuid::parse_str(&owner_user_id)
        .map_err(|error| ApiError::Database(sqlx::Error::Decode(Box::new(error))))?;
    helpers::ensure_owner(&auth, owner_user_id)?;

    let result =
        sqlx::query("delete from profile_creation_request where profile_creation_request_id = $1")
            .bind(db_uuid(request_id))
            .execute(&state.db)
            .await?;

    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound);
    }

    Ok(StatusCode::NO_CONTENT)
}
