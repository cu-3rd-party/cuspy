use crate::AppState;
use crate::api::models::{db_json, db_uuid};
use crate::api::models::ApiError;
use crate::api::models::auth::{AuthResponse, AuthUserRecord, LoginRequest, RegisterRequest};
use crate::api::models::user::UserRecord;
use crate::api::{helpers, user};
use axum::Json;
use axum::extract::State;
use http::{HeaderMap, StatusCode};
use uuid::Uuid;

pub async fn register(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<AuthResponse>), ApiError> {
    let telegram_user_id = helpers::optional_telegram_user_id(&headers, &state)?;

    let (login_identifier, password_hash, telegram_id) = if let Some(telegram_user_id) = telegram_user_id {
        (telegram_user_id.to_string(), None, telegram_user_id)
    } else {
        let email = payload
            .email
            .as_deref()
            .unwrap_or_default()
            .trim()
            .to_lowercase();
        let password = payload.password.as_deref().unwrap_or_default();
        if email.is_empty() || password.len() < 8 {
            return Err(ApiError::BadRequest(
                "email must be present and password must be at least 8 characters".into(),
            ));
        }

        let telegram_id = payload.telegram_id.ok_or(ApiError::BadRequest(
            "telegram_id is required when Telegram auth is disabled".into(),
        ))?;
        (email, Some(helpers::hash_password(password)?), telegram_id)
    };

    let agent_data = helpers::normalize_profile_data(payload.agent_data)?;

    let mut tx = state.db.begin().await?;

    let user_id = Uuid::now_v7();
    let user = sqlx::query_as::<_, UserRecord>(
        r#"
        insert into "user" (user_id, telegram_id, agent_name, agent_data)
        values ($1, $2, $3, $4)
        returning user_id, telegram_id, agent_name, agent_data, is_admin, created_at, updated_at
        "#,
    )
    .bind(db_uuid(user_id))
    .bind(telegram_id)
    .bind(payload.agent_name)
    .bind(db_json(&agent_data))
    .fetch_one(&mut *tx)
    .await?;

    sqlx::query(
        r#"
        insert into rating_history (rating_history_id, user_id, rating, change, reason)
        values ($1, $2, $3, $4, $5)
        "#,
    )
    .bind(db_uuid(Uuid::now_v7()))
    .bind(db_uuid(user_id))
    .bind(helpers::DEFAULT_RATING)
    .bind(helpers::DEFAULT_RATING)
    .bind("initial_rating")
    .execute(&mut *tx)
    .await?;

    let auth_user = match sqlx::query_as::<_, AuthUserRecord>(
        r#"
        insert into auth_user (auth_user_id, user_id, login_identifier, password_hash)
        values ($1, $2, $3, $4)
        returning auth_user_id, user_id, login_identifier, password_hash
        "#,
    )
    .bind(db_uuid(Uuid::now_v7()))
    .bind(db_uuid(user_id))
    .bind(login_identifier.clone())
    .bind(password_hash)
    .fetch_one(&mut *tx)
    .await
    {
        Ok(user) => user,
        Err(sqlx::Error::Database(db_err)) if db_err.code().as_deref() == Some("23505") => {
            return Err(ApiError::BadRequest(format!(
                "user with identifier {login_identifier} already exists"
            )));
        }
        Err(e) => return Err(e.into()),
    };

    tx.commit().await?;

    let access_token = helpers::create_access_token(&state, &auth_user, user.is_admin)?;
    let rating = helpers::fetch_current_rating(&state.db, user.user_id).await?;
    Ok((
        StatusCode::CREATED,
        Json(AuthResponse {
            access_token,
            user: helpers::to_user_response(user, rating),
        }),
    ))
}

pub async fn login(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, ApiError> {
    let telegram_user_id = helpers::optional_telegram_user_id(&headers, &state)?;
    let login_identifier = if let Some(telegram_user_id) = telegram_user_id {
        telegram_user_id.to_string()
    } else {
        payload
            .email
            .as_deref()
            .unwrap_or_default()
            .trim()
            .to_lowercase()
    };

    if login_identifier.is_empty() {
        return Err(ApiError::BadRequest(
            "email is required when Telegram auth is disabled".into(),
        ));
    }

    let auth_user = sqlx::query_as::<_, AuthUserRecord>(
        r#"
        select auth_user_id, user_id, login_identifier, password_hash
        from auth_user
        where login_identifier = $1
        "#,
    )
    .bind(login_identifier)
    .fetch_optional(&state.db)
    .await?
    .ok_or(ApiError::Unauthorized)?;

    if telegram_user_id.is_some() {
        let user_telegram_id =
            sqlx::query_scalar::<_, i64>("select telegram_id from \"user\" where user_id = $1")
                .bind(db_uuid(auth_user.user_id))
                .fetch_optional(&state.db)
                .await?
                .ok_or(ApiError::Unauthorized)?;
        if auth_user.login_identifier != user_telegram_id.to_string() {
            return Err(ApiError::Forbidden);
        }
    }

    if telegram_user_id.is_none() {
        helpers::verify_password(
            auth_user
                .password_hash
                .as_deref()
                .ok_or(ApiError::Unauthorized)?,
            payload.password.as_deref().unwrap_or_default(),
        )?;
    }
    let user = user::fetch_user(&state, auth_user.user_id).await?;
    let access_token = helpers::create_access_token(&state, &auth_user, user.is_admin)?;
    let rating = helpers::fetch_current_rating(&state.db, user.user_id).await?;

    Ok(Json(AuthResponse {
        access_token,
        user: helpers::to_user_response(user, rating),
    }))
}
