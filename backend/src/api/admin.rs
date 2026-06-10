mod agent_data;

use crate::AppState;
use crate::api::models::ApiError;
use crate::api::models::profile::{
    AdminUpdateProfileCreationRequest, ProfileCreationRequestRecord, ProfileCreationRequestResponse,
};
use crate::api::models::user::{CreateUserRequest, UpdateUserRequest, UserRecord, UserResponse};
use crate::api::models::{db_json, db_optional_timestamp, db_uuid};
use crate::api::{helpers, user};
use crate::notifier;
use axum::Json;
use axum::extract::{Path, State};
use http::{HeaderMap, StatusCode};
use uuid::Uuid;

pub async fn admin_list_users(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Vec<UserResponse>>, ApiError> {
    helpers::require_admin(&headers, &state)?;
    let users = sqlx::query_as::<_, UserRecord>(
        r#"
        select
            cast(user_id as text) as user_id,
            telegram_id,
            agent_name,
            rating,
            is_admin,
            cast(created_at as text) as created_at,
            cast(updated_at as text) as updated_at
        from "user"
        order by created_at desc
        "#,
    )
    .fetch_all(&state.db)
    .await?;

    Ok(Json(users.iter().map(|u| u.into()).collect()))
}

pub async fn admin_create_user(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<UserResponse>), ApiError> {
    helpers::require_admin(&headers, &state)?;
    let user = sqlx::query_as::<_, UserRecord>(
        r#"
        insert into "user" (user_id, telegram_id, agent_name, is_admin)
        values (cast($1 as uuid), $2, $3, $4)
        returning
            cast(user_id as text) as user_id,
            telegram_id,
            agent_name,
            is_admin,
            cast(created_at as text) as created_at,
            cast(updated_at as text) as updated_at
        "#,
    )
    .bind(db_uuid(Uuid::now_v7()))
    .bind(payload.telegram_id)
    .bind(payload.agent_name)
    .bind(payload.is_admin.unwrap_or(false))
    .fetch_one(&state.db)
    .await?;

    Ok((
        StatusCode::CREATED,
        Json(helpers::to_user_response(user)),
    ))
}

pub async fn admin_get_user(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(user_id): Path<Uuid>,
) -> Result<Json<UserResponse>, ApiError> {
    helpers::require_admin(&headers, &state)?;
    let user = helpers::fetch_user(&state.db, user_id).await?;
    Ok(Json(helpers::to_user_response(user)))
}

pub async fn admin_update_user(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(user_id): Path<Uuid>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>, ApiError> {
    helpers::require_admin(&headers, &state)?;

    let user = sqlx::query_as::<_, UserRecord>(
        r#"
        update "user"
        set
            telegram_id = coalesce($2, telegram_id),
            agent_name = coalesce($3, agent_name),
            is_admin = coalesce($4, is_admin)
        where user_id = cast($1 as uuid)
        returning
            cast(user_id as text) as user_id,
            telegram_id,
            agent_name,
            is_admin,
            cast(created_at as text) as created_at,
            cast(updated_at as text) as updated_at
        "#,
    )
    .bind(db_uuid(user_id))
    .bind(payload.telegram_id)
    .bind(payload.agent_name)
    .bind(payload.is_admin)
    .fetch_optional(&state.db)
    .await?
    .ok_or(ApiError::NotFound)?;

    Ok(Json(helpers::to_user_response(user)))
}

pub async fn admin_delete_user(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(user_id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    helpers::require_admin(&headers, &state)?;
    let result = sqlx::query(
        r#"
        delete from "user"
        where user_id = cast($1 as uuid)
        "#
    )
        .bind(db_uuid(user_id))
        .execute(&state.db)
        .await?;

    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound);
    }

    Ok(StatusCode::NO_CONTENT)
}

pub async fn admin_list_profile_creation_requests(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Vec<ProfileCreationRequestResponse>>, ApiError> {
    helpers::require_admin(&headers, &state)?;
    let requests = sqlx::query_as::<_, ProfileCreationRequestRecord>(
        r#"
        select
            profile_creation_request_id,
            user_id,
            requested_profile_data_id,
            status,
            reviewer_note,
            reviewed_at,
            created_at,
            updated_at
        from profile_creation_request
        order by created_at desc
        "#,
    )
    .fetch_all(&state.db)
    .await?;

    Ok(Json(
        requests
            .into_iter()
            .map(helpers::to_profile_creation_request_response)
            .collect(),
    ))
}

pub async fn admin_get_profile_creation_request(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(request_id): Path<Uuid>,
) -> Result<Json<ProfileCreationRequestResponse>, ApiError> {
    helpers::require_admin(&headers, &state)?;
    let request = sqlx::query_as::<_, ProfileCreationRequestRecord>(
        r#"
        select
            profile_creation_request_id,
            user_id,
            requested_profile_data_id,
            status,
            reviewer_note,
            reviewed_at,
            created_at,
            updated_at
        from profile_creation_request
        where profile_creation_request_id = cast($1 as uuid)
        "#,
    )
    .bind(db_uuid(request_id))
    .fetch_optional(&state.db)
    .await?
    .ok_or(ApiError::NotFound)?;

    Ok(Json(helpers::to_profile_creation_request_response(request)))
}

pub async fn admin_update_profile_creation_request(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(request_id): Path<Uuid>,
    Json(payload): Json<AdminUpdateProfileCreationRequest>,
) -> Result<Json<ProfileCreationRequestResponse>, ApiError> {
    helpers::require_admin(&headers, &state)?;

    if let Some(status) = payload.status.as_deref()
        && !matches!(status, "sent" | "confirmed" | "rejected")
    {
        return Err(ApiError::BadRequest("invalid status".into()));
    }

    let reviewed_at = payload
        .status
        .as_deref()
        .filter(|status| matches!(*status, "confirmed" | "rejected"))
        .map(|_| sqlx::types::time::OffsetDateTime::now_utc());

    let mut tx = state.db.begin().await?;

    let request = sqlx::query_as::<_, ProfileCreationRequestRecord>(
        r#"
        update profile_creation_request
        set
            requested_profile_data_id = coalesce($1, requested_profile_data_id),
            status = coalesce($2, status),
            reviewer_note = coalesce($3, reviewer_note),
            reviewed_at = coalesce($4, reviewed_at)
        where profile_creation_request_id = cast($1 as uuid)
        returning
            profile_creation_request_id,
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
    .bind(payload.status.clone())
    .bind(payload.reviewer_note)
    .bind(db_optional_timestamp(reviewed_at))
    .fetch_optional(&mut *tx)
    .await?
    .ok_or(ApiError::NotFound)?;

    if payload.status.as_deref() == Some("confirmed") {
        sqlx::query(
            r#"
            update "user"
            set agent_data_id = $2
            where user_id = $1
            "#,
        )
        .bind(db_uuid(request.user_id))
        .bind(db_uuid(request.requested_profile_data_id))
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;

    if request.status == "confirmed" {
        notifier::notify_user(
            &state,
            request.user_id,
            "Profile approved. Full operative access live. Open target intel and resume hunt.",
        )
        .await;
    } else if request.status == "rejected" {
        let note = request
            .reviewer_note
            .as_deref()
            .unwrap_or("No reviewer note attached.");
        notifier::notify_user(
            &state,
            request.user_id,
            format!("Profile rejected. Edit dossier and resend. Reviewer note: {note}"),
        )
        .await;
    }

    Ok(Json(helpers::to_profile_creation_request_response(request)))
}

pub async fn admin_delete_profile_creation_request(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(request_id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    helpers::require_admin(&headers, &state)?;
    let result =
        sqlx::query(
            r#"
            delete from 
                profile_creation_request 
            where profile_creation_request_id = $1
            "#)
            .bind(db_uuid(request_id))
            .execute(&state.db)
            .await?;

    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound);
    }

    Ok(StatusCode::NO_CONTENT)
}
