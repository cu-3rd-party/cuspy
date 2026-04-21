use axum::extract::State;
use http::{HeaderMap, StatusCode};
use axum::Json;
use uuid::Uuid;
use crate::api::{helpers, user};
use crate::api::models::ApiError;
use crate::api::models::auth::{AuthResponse, AuthUserRecord, LoginRequest, RegisterRequest};
use crate::AppState;
use crate::api::models::user::UserRecord;

pub async fn register(
    State(state): State<AppState>,
    #[cfg_attr(not(feature = "telegram-auth"), allow(unused_variables))] headers: HeaderMap,
    Json(payload): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<AuthResponse>), ApiError> {
    #[cfg(feature = "telegram-auth")]
    {
        let telegram = helpers::verify_telegram_init_data(&headers, &state)?;
        if telegram.telegram_user_id != payload.telegram_id {
            return Err(ApiError::Forbidden);
        }
    }

    let email = payload.email.trim().to_lowercase();
    if email.is_empty() || payload.password.len() < 8 {
        return Err(ApiError::BadRequest(
            "email must be present and password must be at least 8 characters".into(),
        ));
    }

    let agent_data = helpers::normalize_profile_data(payload.agent_data)?;
    let password_hash = helpers::hash_password(&payload.password)?;

    let mut tx = state.db.begin().await?;

    let user_id = Uuid::now_v7();
    let user = sqlx::query_as::<_, UserRecord>(
        r#"
        insert into "user" (user_id, telegram_id, rating, agent_name, agent_data)
        values ($1, $2, $3, $4, $5)
        returning user_id, telegram_id, rating, agent_name, agent_data, created_at, updated_at
        "#,
    )
    .bind(user_id)
    .bind(payload.telegram_id)
    .bind(payload.rating)
    .bind(payload.agent_name)
    .bind(agent_data)
    .fetch_one(&mut *tx)
    .await?;

    let auth_user = sqlx::query_as::<_, AuthUserRecord>(
        r#"
        insert into auth_user (auth_user_id, user_id, email, password_hash)
        values ($1, $2, $3, $4)
        returning auth_user_id, user_id, email, password_hash
        "#,
    )
    .bind(Uuid::now_v7())
    .bind(user_id)
    .bind(email)
    .bind(password_hash)
    .fetch_one(&mut *tx)
    .await?;

    tx.commit().await?;

    let access_token = helpers::create_access_token(&state, &auth_user)?;
    Ok((
        StatusCode::CREATED,
        Json(AuthResponse {
            access_token,
            user: helpers::to_user_response(user),
        }),
    ))
}

pub async fn login(
    State(state): State<AppState>,
    #[cfg_attr(not(feature = "telegram-auth"), allow(unused_variables))] headers: HeaderMap,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, ApiError> {
    let auth_user = sqlx::query_as::<_, AuthUserRecord>(
        r#"
        select auth_user_id, user_id, email, password_hash
        from auth_user
        where email = $1
        "#,
    )
    .bind(payload.email.trim().to_lowercase())
    .fetch_optional(&state.db)
    .await?
    .ok_or(ApiError::Unauthorized)?;

    #[cfg(feature = "telegram-auth")]
    {
        let telegram = helpers::verify_telegram_init_data(&headers, &state)?;
        let user_telegram_id = sqlx::query_scalar::<_, i64>("select telegram_id from \"user\" where user_id = $1")
            .bind(auth_user.user_id)
            .fetch_optional(&state.db)
            .await?
            .ok_or(ApiError::Unauthorized)?;
        if telegram.telegram_user_id != user_telegram_id {
            return Err(ApiError::Forbidden);
        }
    }

    helpers::verify_password(&auth_user.password_hash, &payload.password)?;
    let user = user::fetch_user(&state, auth_user.user_id).await?;
    let access_token = helpers::create_access_token(&state, &auth_user)?;

    Ok(Json(AuthResponse {
        access_token,
        user: helpers::to_user_response(user),
    }))
}
