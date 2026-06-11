use crate::api::extractor::AuthUser;
use crate::api::models::auth::{AuthResponse, AuthUserRecord, LoginRequest};
use crate::api::models::{db_uuid, ApiError};
use crate::api::helpers;
use crate::ApiContext;
use axum::extract::State;
use axum::Json;
use sqlx::Row;

pub async fn login(
    State(state): State<ApiContext>,
    AuthUser(user): AuthUser,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, ApiError> {
    let login_identifier: String;
    #[cfg(feature = "telegram-auth")]
    {
        login_identifier = user.tg.user.id.to_string();
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
    let user = helpers::fetch_user(&state.db, user.user_id).await?;

    Ok(Json(AuthResponse {
        access_token,
        user: helpers::to_user_response(user),
    }))
}
