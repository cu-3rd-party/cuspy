use axum::extract::{Path, State};
use http::{HeaderMap, StatusCode};
use uuid::Uuid;
use axum::Json;
use serde_json::Value;
use crate::api::helpers;
use crate::api::models::ApiError;
use crate::api::models::similarity::{SimilarityRequest, SimilarityResponse};
use crate::api::models::user::{UpdateUserRequest, UserRecord, UserResponse};
use crate::AppState;

pub async fn fetch_user(state: &AppState, user_id: Uuid) -> Result<UserRecord, ApiError> {
    sqlx::query_as::<_, UserRecord>(
        r#"
        select user_id, telegram_id, rating, agent_name, agent_data, created_at, updated_at
        from "user"
        where user_id = $1
        "#,
    )
    .bind(user_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or(ApiError::NotFound)
}

pub async fn me(State(state): State<AppState>, headers: HeaderMap) -> Result<Json<UserResponse>, ApiError> {
    #[cfg(feature = "telegram-auth")]
    helpers::verify_telegram_init_data(&headers, &state)?;
    let auth = helpers::require_bearer_token(&headers, &state)?;
    let user = fetch_user(&state, auth.user_id).await?;
    Ok(Json(helpers::to_user_response(user)))
}

pub async fn get_user(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(user_id): Path<Uuid>,
) -> Result<Json<UserResponse>, ApiError> {
    #[cfg(feature = "telegram-auth")]
    helpers::verify_telegram_init_data(&headers, &state)?;
    let auth = helpers::require_bearer_token(&headers, &state)?;
    helpers::ensure_owner(&auth, user_id)?;
    let user = fetch_user(&state, user_id).await?;
    Ok(Json(helpers::to_user_response(user)))
}

pub async fn update_user(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(user_id): Path<Uuid>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>, ApiError> {
    #[cfg(feature = "telegram-auth")]
    helpers::verify_telegram_init_data(&headers, &state)?;
    let auth = helpers::require_bearer_token(&headers, &state)?;
    helpers::ensure_owner(&auth, user_id)?;

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

pub async fn delete_user(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(user_id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    #[cfg(feature = "telegram-auth")]
    helpers::verify_telegram_init_data(&headers, &state)?;
    let auth = helpers::require_bearer_token(&headers, &state)?;
    helpers::ensure_owner(&auth, user_id)?;

    let result = sqlx::query(r#"delete from "user" where user_id = $1"#)
        .bind(user_id)
        .execute(&state.db)
        .await?;

    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound);
    }

    Ok(StatusCode::NO_CONTENT)
}

pub async fn compare_user_profiles(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path((left_user_id, right_user_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<SimilarityResponse>, ApiError> {
    #[cfg(feature = "telegram-auth")]
    helpers::verify_telegram_init_data(&headers, &state)?;
    let auth = helpers::require_bearer_token(&headers, &state)?;
    helpers::ensure_owner(&auth, left_user_id)?;
    helpers::ensure_owner(&auth, right_user_id)?;

    let left = sqlx::query_scalar::<_, Value>(r#"select agent_data from "user" where user_id = $1"#)
        .bind(left_user_id)
        .fetch_optional(&state.db)
        .await?
        .ok_or(ApiError::NotFound)?;
    let right = sqlx::query_scalar::<_, Value>(r#"select agent_data from "user" where user_id = $1"#)
        .bind(right_user_id)
        .fetch_optional(&state.db)
        .await?
        .ok_or(ApiError::NotFound)?;

    Ok(Json(helpers::compare_profile_similarity(&left, &right)?))
}

pub async fn compare_profiles(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<SimilarityRequest>,
) -> Result<Json<SimilarityResponse>, ApiError> {
    #[cfg(feature = "telegram-auth")]
    helpers::verify_telegram_init_data(&headers, &state)?;
    let _auth = helpers::require_bearer_token(&headers, &state)?;
    Ok(Json(helpers::compare_profile_similarity(&payload.left, &payload.right)?))
}
