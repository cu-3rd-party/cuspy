use crate::ApiContext;
use crate::models::ApiError;
use crate::models::auth::{AuthResponse, AuthUserRecord, LoginRequest};
#[cfg(feature = "telegram-auth")]
use crate::models::db_uuid;
use crate::rest::helpers;
#[cfg(feature = "telegram-auth")]
use crate::telegram;
use axum::Json;
use axum::extract::State;
#[cfg(feature = "telegram-auth")]
use http::HeaderMap;
#[cfg(feature = "telegram-auth")]
use sqlx::Row;

#[utoipa::path(
    post,
    path = "/api/auth/login",
    tag = "auth",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login succeeded", body = AuthResponse),
        (status = 400, description = "Bad request", body = crate::models::ErrorResponse),
        (status = 401, description = "Unauthorized", body = crate::models::ErrorResponse),
        (status = 403, description = "Forbidden", body = crate::models::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::models::ErrorResponse),
    ),
    security(("bearer_auth" = []))
)]
pub async fn login(
    State(state): State<ApiContext>,
    #[cfg(feature = "telegram-auth")] headers: HeaderMap,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, ApiError> {
    #[cfg(feature = "telegram-auth")]
    let _ = &payload;

    let login_identifier: String;
    #[cfg(feature = "telegram-auth")]
    {
        let init_data = headers
            .get("x-telegram-init-data")
            .and_then(|value| value.to_str().ok())
            .and_then(|value| {
                telegram::TelegramInitData::from_header(&state.telegram_bot_token, value)
            })
            .ok_or(ApiError::Unauthorized)?;
        login_identifier = init_data.user.id.to_string();
    }
    #[cfg(not(feature = "telegram-auth"))]
    {
        login_identifier = payload
            .email
            .as_deref()
            .unwrap_or_default()
            .trim()
            .to_lowercase();

        if login_identifier.is_empty() {
            return Err(ApiError::BadRequest(
                "email is required when Telegram auth is disabled".into(),
            ));
        }
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

    #[cfg(feature = "telegram-auth")]
    {
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
    #[cfg(not(feature = "telegram-auth"))]
    {
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

    Ok(Json(AuthResponse {
        access_token,
        user: user.into_response(&state.db).await?,
    }))
}
