use crate::ApiContext;
use crate::api::helpers;
use crate::api::models::auth::{AuthResponse, AuthUserRecord, LoginRequest};
use crate::api::models::{ApiError, db_uuid};
use axum::Json;
use axum::extract::State;
use http::HeaderMap;
use sqlx::Row;

pub async fn login(
    State(state): State<ApiContext>,
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
        select
            cast(auth_user_id as text) as auth_user_id,
            cast(user_id as text) as user_id,
            login_identifier,
            password_hash
        from auth_user
        where login_identifier = $1
        "#,
    )
    .bind(login_identifier)
    .fetch_optional(&state.db)
    .await?
    .ok_or(ApiError::Unauthorized)?;

    if telegram_user_id.is_some() {
        let user_telegram_id: i64 = sqlx::query(
            r#"
            select 
                telegram_id
            from "user"
            where user_id = cast($1 as uuid)
            "#,
        )
        .bind(db_uuid(auth_user.user_id))
        .fetch_optional(&state.db)
        .await?
        .ok_or(ApiError::Unauthorized)?
        .try_get("telegram_id")?;
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
    let user = helpers::fetch_user(&state.db, auth_user.user_id).await?;
    let access_token = helpers::create_access_token(&state, &auth_user, user.is_admin)?;
    let user = helpers::fetch_user(&state.db, user.user_id).await?;

    Ok(Json(AuthResponse {
        access_token,
        user: helpers::to_user_response(user),
    }))
}
