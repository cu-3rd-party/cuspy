use crate::AppState;
use crate::api::helpers;
use crate::api::models::{db_json, db_uuid};
use crate::api::models::ApiError;
use crate::api::models::similarity::{SimilarityRequest, SimilarityResponse};
use crate::api::models::user::{UpdateUserRequest, UserRecord, UserResponse};
use axum::Json;
use axum::extract::{Path, State};
use http::{HeaderMap, StatusCode};
use serde_json::Value;
use uuid::Uuid;

pub async fn fetch_user(state: &AppState, user_id: Uuid) -> Result<UserRecord, ApiError> {
    sqlx::query_as::<_, UserRecord>(state.db_param(
        r#"
        select
            cast(user_id as text) as user_id,
            telegram_id,
            agent_name,
            cast(agent_data as text) as agent_data,
            is_admin,
            cast(created_at as text) as created_at,
            cast(updated_at as text) as updated_at
        from "user"
        where user_id = $1
        "#,
        r#"
        select
            cast(user_id as text) as user_id,
            telegram_id,
            agent_name,
            cast(agent_data as text) as agent_data,
            is_admin,
            cast(created_at as text) as created_at,
            cast(updated_at as text) as updated_at
        from "user"
        where user_id = cast($1 as uuid)
        "#,
    ))
    .bind(db_uuid(user_id))
    .fetch_optional(&state.db)
    .await?
    .ok_or(ApiError::NotFound)
}

pub async fn me(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<UserResponse>, ApiError> {
    helpers::optional_telegram_user_id(&headers, &state)?;
    let auth = helpers::require_bearer_token(&headers, &state)?;
    let user = fetch_user(&state, auth.user_id).await?;
    let rating = helpers::fetch_current_rating(&state.db, user.user_id).await?;
    Ok(Json(helpers::to_user_response(user, rating)))
}

pub async fn get_user(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(user_id): Path<Uuid>,
) -> Result<Json<UserResponse>, ApiError> {
    helpers::optional_telegram_user_id(&headers, &state)?;
    let auth = helpers::require_bearer_token(&headers, &state)?;
    helpers::ensure_owner(&auth, user_id)?;
    let user = fetch_user(&state, user_id).await?;
    let rating = helpers::fetch_current_rating(&state.db, user.user_id).await?;
    Ok(Json(helpers::to_user_response(user, rating)))
}

pub async fn update_user(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(user_id): Path<Uuid>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>, ApiError> {
    helpers::optional_telegram_user_id(&headers, &state)?;
    let auth = helpers::require_bearer_token(&headers, &state)?;
    helpers::ensure_owner(&auth, user_id)?;

    let agent_data = match payload.agent_data {
        Some(value) => Some(helpers::normalize_profile_data(Some(value))?),
        None => None,
    };

    let user = sqlx::query_as::<_, UserRecord>(state.db_param(
        r#"
        update "user"
        set
            telegram_id = coalesce($2, telegram_id),
            agent_name = coalesce($3, agent_name),
            agent_data = coalesce($4, agent_data)
        where user_id = $1
        returning
            cast(user_id as text) as user_id,
            telegram_id,
            agent_name,
            cast(agent_data as text) as agent_data,
            is_admin,
            cast(created_at as text) as created_at,
            cast(updated_at as text) as updated_at
        "#,
        r#"
        update "user"
        set
            telegram_id = coalesce($2, telegram_id),
            agent_name = coalesce($3, agent_name),
            agent_data = coalesce(cast($4 as jsonb), agent_data)
        where user_id = cast($1 as uuid)
        returning
            cast(user_id as text) as user_id,
            telegram_id,
            agent_name,
            cast(agent_data as text) as agent_data,
            is_admin,
            cast(created_at as text) as created_at,
            cast(updated_at as text) as updated_at
        "#,
    ))
    .bind(db_uuid(user_id))
    .bind(payload.telegram_id)
    .bind(payload.agent_name)
    .bind(agent_data.as_ref().map(db_json))
    .fetch_optional(&state.db)
    .await?
    .ok_or(ApiError::NotFound)?;

    let rating = helpers::fetch_current_rating(&state.db, user.user_id).await?;
    Ok(Json(helpers::to_user_response(user, rating)))
}

pub async fn update_me(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>, ApiError> {
    let auth = helpers::require_bearer_token(&headers, &state)?;

    update_user(
        State(state),
        headers,
        Path(auth.user_id),
        Json(UpdateUserRequest {
            is_admin: None,
            ..payload
        }),
    )
    .await
}

pub async fn delete_user(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(user_id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    helpers::optional_telegram_user_id(&headers, &state)?;
    let auth = helpers::require_bearer_token(&headers, &state)?;
    helpers::ensure_owner(&auth, user_id)?;

    let result = sqlx::query(state.db_param(
        r#"delete from "user" where user_id = $1"#,
        r#"delete from "user" where user_id = cast($1 as uuid)"#,
    ))
        .bind(db_uuid(user_id))
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
    helpers::optional_telegram_user_id(&headers, &state)?;
    let auth = helpers::require_bearer_token(&headers, &state)?;
    helpers::ensure_owner(&auth, left_user_id)?;
    helpers::ensure_owner(&auth, right_user_id)?;

    let left =
        sqlx::query_scalar::<_, String>(state.db_param(
            r#"select agent_data from "user" where user_id = $1"#,
            r#"select cast(agent_data as text) from "user" where user_id = cast($1 as uuid)"#,
        ))
            .bind(db_uuid(left_user_id))
            .fetch_optional(&state.db)
            .await?
            .ok_or(ApiError::NotFound)?;
    let right =
        sqlx::query_scalar::<_, String>(state.db_param(
            r#"select agent_data from "user" where user_id = $1"#,
            r#"select cast(agent_data as text) from "user" where user_id = cast($1 as uuid)"#,
        ))
            .bind(db_uuid(right_user_id))
            .fetch_optional(&state.db)
            .await?
            .ok_or(ApiError::NotFound)?;

    let left: Value = serde_json::from_str(&left)
        .map_err(|error| ApiError::Database(sqlx::Error::Decode(Box::new(error))))?;
    let right: Value = serde_json::from_str(&right)
        .map_err(|error| ApiError::Database(sqlx::Error::Decode(Box::new(error))))?;

    Ok(Json(helpers::compare_profile_similarity(&left, &right)?))
}

pub async fn compare_profiles(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<SimilarityRequest>,
) -> Result<Json<SimilarityResponse>, ApiError> {
    helpers::optional_telegram_user_id(&headers, &state)?;
    let _auth = helpers::require_bearer_token(&headers, &state)?;
    Ok(Json(helpers::compare_profile_similarity(
        &payload.left,
        &payload.right,
    )?))
}
