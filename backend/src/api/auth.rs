use crate::AppState;
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
    #[cfg_attr(not(feature = "telegram-auth"), allow(unused_variables))] headers: HeaderMap,
    Json(payload): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<AuthResponse>), ApiError> {
    #[cfg(feature = "telegram-auth")]
    let telegram_user_id = {
        let telegram = helpers::verify_telegram_init_data(&headers, &state)?;
        telegram.telegram_user_id
    };

    #[cfg(feature = "telegram-auth")]
    let login_identifier = telegram_user_id.to_string();

    #[cfg(not(feature = "telegram-auth"))]
    let email = payload.email.trim().to_lowercase();
    #[cfg(not(feature = "telegram-auth"))]
    if email.is_empty() || payload.password.len() < 8 {
        return Err(ApiError::BadRequest(
            "email must be present and password must be at least 8 characters".into(),
        ));
    }

    #[cfg(not(feature = "telegram-auth"))]
    let login_identifier = email;

    let agent_data = helpers::normalize_profile_data(payload.agent_data)?;
    #[cfg(not(feature = "telegram-auth"))]
    let password_hash = helpers::hash_password(&payload.password)?;
    #[cfg(feature = "telegram-auth")]
    let password_hash: Option<String> = None;
    #[cfg(not(feature = "telegram-auth"))]
    let password_hash = Some(password_hash);

    let mut tx = state.db.begin().await?;

    let user_id = Uuid::now_v7();
    let user = sqlx::query_as::<_, UserRecord>(
        r#"
        insert into "user" (user_id, telegram_id, agent_name, agent_data)
        values ($1, $2, $3, $4)
        returning user_id, telegram_id, agent_name, agent_data, is_admin, created_at, updated_at
        "#,
    )
    .bind(user_id)
    .bind({
        #[cfg(feature = "telegram-auth")]
        {
            telegram_user_id
        }
        #[cfg(not(feature = "telegram-auth"))]
        {
            payload.telegram_id
        }
    })
    .bind(payload.agent_name)
    .bind(agent_data)
    .fetch_one(&mut *tx)
    .await?;

    sqlx::query(
        r#"
        insert into rating_history (rating_history_id, user_id, rating, change, reason)
        values ($1, $2, $3, $4, $5)
        "#,
    )
    .bind(Uuid::now_v7())
    .bind(user_id)
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
    .bind(Uuid::now_v7())
    .bind(user_id)
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
    #[cfg_attr(not(feature = "telegram-auth"), allow(unused_variables))] headers: HeaderMap,
    #[cfg_attr(feature = "telegram-auth", allow(unused_variables))] Json(payload): Json<
        LoginRequest,
    >,
) -> Result<Json<AuthResponse>, ApiError> {
    #[cfg(feature = "telegram-auth")]
    let login_identifier = helpers::verify_telegram_init_data(&headers, &state)?
        .telegram_user_id
        .to_string();

    #[cfg(not(feature = "telegram-auth"))]
    let login_identifier = payload.email.trim().to_lowercase();

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

    #[cfg(feature = "telegram-auth")]
    {
        let user_telegram_id =
            sqlx::query_scalar::<_, i64>("select telegram_id from \"user\" where user_id = $1")
                .bind(auth_user.user_id)
                .fetch_optional(&state.db)
                .await?
                .ok_or(ApiError::Unauthorized)?;
        if auth_user.login_identifier != user_telegram_id.to_string() {
            return Err(ApiError::Forbidden);
        }
    }

    #[cfg(not(feature = "telegram-auth"))]
    helpers::verify_password(
        auth_user
            .password_hash
            .as_deref()
            .ok_or(ApiError::Unauthorized)?,
        &payload.password,
    )?;
    let user = user::fetch_user(&state, auth_user.user_id).await?;
    let access_token = helpers::create_access_token(&state, &auth_user, user.is_admin)?;
    let rating = helpers::fetch_current_rating(&state.db, user.user_id).await?;

    Ok(Json(AuthResponse {
        access_token,
        user: helpers::to_user_response(user, rating),
    }))
}
