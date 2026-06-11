use crate::ApiContext;
use crate::api::{extractor, helpers};
use crate::api::models::auth::{AuthResponse, AuthUserRecord, RegisterRequest};
use crate::api::models::user::UserRecord;
use crate::api::models::{db_uuid, ApiError};
use axum::Json;
use axum::extract::State;
use http::{HeaderMap, StatusCode};
use log::error;
use uuid::Uuid;
use crate::api::extractor::{AuthUser, User};

fn map_register_database_error(error: sqlx::Error, login_identifier: &str) -> ApiError {
    match error {
        sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
            let message = db_err.message();
            if message.contains("auth_user") || message.contains("login_identifier") {
                ApiError::BadRequest(format!(
                    "user with identifier {login_identifier} already exists"
                ))
            } else if message.contains("user.telegram_id") || message.contains("telegram_id") {
                ApiError::BadRequest("user with this telegram_id already exists".into())
            } else {
                ApiError::BadRequest("user already exists".into())
            }
        }
        sqlx::Error::Decode(err) => {
            error!("db error occurred: {}", &err.to_string());
            ApiError::Internal(format!("column not found: {}", err.to_string()).into())
        }
        sqlx::Error::ColumnNotFound(err) => {
            error!("db error occurred: {}", &err);
            ApiError::Internal(format!("column not found: {err}").into())
        }
        other => other.into(),
    }
}

pub async fn register(
    State(state): State<ApiContext>,
    AuthUser(user): AuthUser,
    Json(payload): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<AuthResponse>), ApiError> {
    #[cfg(feature = "telegram-auth")]
    let (login_identifier, password_hash, telegram_id) = {
        let telegram_user_id = user.tg.user.id;
        (telegram_user_id.to_string(), None::<String>, telegram_user_id)
    };

    #[cfg(not(feature = "telegram-auth"))]
    let (login_identifier, password_hash, telegram_id) = {
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

    let mut tx = state.db.begin().await?;

    let user_id = Uuid::now_v7();
    let user = match sqlx::query_as::<_, UserRecord>(
        r#"
        insert into "user" (user_id, telegram_id, agent_name, is_admin)
        values ($1, $2, $3, $4)
        returning
            user_id,
            telegram_id,
            agent_name,
            is_admin,
            created_at,
            updated_at
        "#,
    )
    .bind(db_uuid(user_id))
    .bind(telegram_id)
    .bind(payload.agent_name)
    .bind(user.is_admin)
    .fetch_one(&mut *tx)
    .await
    {
        Ok(row) => row,
        Err(error) => return Err(map_register_database_error(error, &login_identifier)),
    };

    sqlx::query(
        r#"
        insert into rating_history (rating_history_id, user_id, rating, change, reason)
        values (cast($1 as uuid), cast($2 as uuid), $3, $4, $5)
        "#,
    )
    .bind(db_uuid(Uuid::now_v7()))
    .bind(db_uuid(user_id))
    .bind(helpers::DEFAULT_RATING)
    .bind(helpers::DEFAULT_RATING)
    .bind("initial_rating")
    .execute(&mut *tx)
    .await
    .map_err(|error| map_register_database_error(error, &login_identifier))?;

    let auth_user = match sqlx::query_as::<_, AuthUserRecord>(
        r#"
            insert into auth_user (auth_user_id, user_id, login_identifier, password_hash)
            values (cast($1 as uuid), cast($2 as uuid), $3, $4)
            returning
                cast(auth_user_id as text) as auth_user_id,
                cast(user_id as text) as user_id,
                login_identifier,
                password_hash
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
        Err(error) => return Err(map_register_database_error(error, &login_identifier)),
    };

    tx.commit().await?;

    let access_token = helpers::create_access_token(&state, &auth_user, user.is_admin)?;
    let user = helpers::fetch_user(&state.db, user.user_id).await?;
    Ok((
        StatusCode::CREATED,
        Json(AuthResponse {
            access_token,
            user: helpers::to_user_response(user),
        }),
    ))
}
