use axum::extract::{Path, State};
use http::{HeaderMap, StatusCode};
use uuid::Uuid;
use axum::Json;
use crate::api::{helpers, user};
use crate::api::models::ApiError;
use crate::api::models::profile::{AdminUpdateProfileCreationRequest, ProfileCreationRequestRecord, ProfileCreationRequestResponse};
use crate::api::models::user::{CreateUserRequest, UpdateUserRequest, UserRecord, UserResponse};
use crate::AppState;

pub async fn admin_list_users(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Vec<UserResponse>>, ApiError> {
    helpers::require_admin(&headers, &state)?;
    let users = sqlx::query_as::<_, UserRecord>(
        r#"
        select user_id, telegram_id, rating, agent_name, agent_data, created_at, updated_at
        from "user"
        order by created_at desc
        "#,
    )
    .fetch_all(&state.db)
    .await?;
    Ok(Json(users.into_iter().map(helpers::to_user_response).collect()))
}

pub async fn admin_create_user(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<UserResponse>), ApiError> {
    helpers::require_admin(&headers, &state)?;
    let agent_data = helpers::normalize_profile_data(payload.agent_data)?;
    let user = sqlx::query_as::<_, UserRecord>(
        r#"
        insert into "user" (user_id, telegram_id, rating, agent_name, agent_data)
        values ($1, $2, $3, $4, $5)
        returning user_id, telegram_id, rating, agent_name, agent_data, created_at, updated_at
        "#,
    )
    .bind(Uuid::now_v7())
    .bind(payload.telegram_id)
    .bind(payload.rating)
    .bind(payload.agent_name)
    .bind(agent_data)
    .fetch_one(&state.db)
    .await?;

    Ok((StatusCode::CREATED, Json(helpers::to_user_response(user))))
}

pub async fn admin_get_user(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(user_id): Path<Uuid>,
) -> Result<Json<UserResponse>, ApiError> {
    helpers::require_admin(&headers, &state)?;
    let user = user::fetch_user(&state, user_id).await?;
    Ok(Json(helpers::to_user_response(user)))
}

pub async fn admin_update_user(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(user_id): Path<Uuid>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>, ApiError> {
    helpers::require_admin(&headers, &state)?;
    let agent_data = match payload.agent_data {
        Some(value) => Some(helpers::normalize_profile_data(Some(value))?),
        None => None,
    };

    let user = sqlx::query_as::<_, UserRecord>(
        r#"
        update "user"
        set
            telegram_id = coalesce($2, telegram_id),
            rating = coalesce($3, rating),
            agent_name = coalesce($4, agent_name),
            agent_data = coalesce($5, agent_data)
        where user_id = $1
        returning user_id, telegram_id, rating, agent_name, agent_data, created_at, updated_at
        "#,
    )
    .bind(user_id)
    .bind(payload.telegram_id)
    .bind(payload.rating)
    .bind(payload.agent_name)
    .bind(agent_data)
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
    let result = sqlx::query(r#"delete from "user" where user_id = $1"#)
        .bind(user_id)
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
            requested_profile_data,
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
    .bind(request_id)
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

    let requested_profile_data = match payload.requested_profile_data {
        Some(value) => Some(helpers::normalize_profile_data(Some(value))?),
        None => None,
    };

    if let Some(status) = payload.status.as_deref() {
        if !matches!(status, "sent" | "confirmed" | "rejected") {
            return Err(ApiError::BadRequest("invalid status".into()));
        }
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
            requested_profile_data = coalesce($2, requested_profile_data),
            status = coalesce($3, status),
            reviewer_note = coalesce($4, reviewer_note),
            reviewed_at = case when $5 is not null then $5 else reviewed_at end
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
    .bind(request_id)
    .bind(requested_profile_data)
    .bind(payload.status.clone())
    .bind(payload.reviewer_note)
    .bind(reviewed_at)
    .fetch_optional(&mut *tx)
    .await?
    .ok_or(ApiError::NotFound)?;

    if payload.status.as_deref() == Some("confirmed") {
        sqlx::query(
            r#"
            update "user"
            set agent_data = $2
            where user_id = $1
            "#,
        )
        .bind(request.user_id)
        .bind(request.requested_profile_data.clone())
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;
    Ok(Json(helpers::to_profile_creation_request_response(request)))
}

pub async fn admin_delete_profile_creation_request(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(request_id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    helpers::require_admin(&headers, &state)?;
    let result = sqlx::query("delete from profile_creation_request where profile_creation_request_id = $1")
        .bind(request_id)
        .execute(&state.db)
        .await?;

    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound);
    }

    Ok(StatusCode::NO_CONTENT)
}