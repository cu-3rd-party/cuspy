use crate::models::user::User;
use crate::models::{ApiError, db_optional_uuid, db_uuid, parse_optional_uuid, parse_uuid};
use crate::rest::helpers;
use rand::RngExt;
use rand::distr::SampleString;
use serde::{Deserialize, Serialize};
use sqlx::{Executor, FromRow, Postgres, postgres::PgRow};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize, ToSchema)]
pub struct EmailRegisterRequest {
    pub email: String,
    pub password: String,
    pub username: Option<String>, // unused for now
}

#[derive(Deserialize, ToSchema)]
pub struct EmailLoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, ToSchema)]
pub struct TelegramInitDataRequest {
    pub init_data: String,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct AuthTokenPair {
    pub access_token: String,
    pub refresh_token: String,
}

pub struct AuthUserRecord {
    pub auth_user_id: Uuid,
    pub user_id: Option<Uuid>, // это нул когда человек открыл приложение но не получил еще рейтинг
    // TODO: probably create a trait login identifier and abstract out different login methods behind it
    pub telegram_id: Option<i64>,
    pub email: Option<String>,
    pub password_hash: Option<String>,
}

impl Default for AuthUserRecord {
    fn default() -> Self {
        Self {
            auth_user_id: Uuid::new_v4(),
            user_id: None,
            telegram_id: None,
            email: Some(format!(
                "{}@default.com",
                rand::distr::Alphabetic.sample_string(&mut rand::rng(), 5),
            )),
            password_hash: Some(helpers::hash_password("devpassword").unwrap()),
        }
    }
}

impl AuthUserRecord {
    pub async fn new_telegram_user<'c, E>(
        executor: E,
        user_id: Option<Uuid>,
        telegram_id: i64,
    ) -> Result<Self, ApiError>
    where
        E: Executor<'c, Database = Postgres>,
    {
        Ok(sqlx::query_as(
            r#"
                insert into "auth_user" (user_id, telegram_id)
                values (cast($1 as uuid), $2)
                returning *
            "#,
        )
        .bind(db_optional_uuid(user_id))
        .bind(telegram_id)
        .fetch_one(executor)
        .await?)
    }

    pub async fn new_email_user<'c, E>(
        executor: E,
        user_id: Option<Uuid>,
        email: String,
        password: String,
    ) -> Result<Self, ApiError>
    where
        E: Executor<'c, Database = Postgres>,
    {
        let password_hash = helpers::hash_password(&password)?;
        Ok(sqlx::query_as(
            r#"
                insert into "auth_user" (user_id, email, password_hash)
                values (cast($1 as uuid), $2, $3)
                returning *
            "#,
        )
        .bind(db_optional_uuid(user_id))
        .bind(email)
        .bind(password_hash)
        .fetch_one(executor)
        .await?)
    }

    pub async fn get_by_id<'c, E>(executor: E, auth_user_id: Uuid) -> Option<Self>
    where
        E: Executor<'c, Database = Postgres>,
    {
        sqlx::query_as(
            r#"
                select *
                from auth_user
                where auth_user_id = cast($1 as uuid)
                limit 1
            "#,
        )
        .bind(db_uuid(auth_user_id))
        .fetch_optional(executor)
        .await
        .ok()
        .flatten()
    }

    pub async fn get_by_user_id<'c, E>(executor: E, user_id: Uuid) -> Option<Vec<Self>>
    where
        E: Executor<'c, Database = Postgres>,
    {
        sqlx::query_as(
            r#"
                select *
                from auth_user
                where user_id = cast($1 as uuid)
            "#,
        )
        .bind(db_uuid(user_id))
        .fetch_all(executor)
        .await
        .ok()
    }

    pub async fn get_by_email<'c, E>(executor: E, email: String) -> Option<Self>
    where
        E: Executor<'c, Database = Postgres>,
    {
        sqlx::query_as(
            r#"
                select *
                from auth_user
                where email = $1
            "#,
        )
        .bind(email)
        .fetch_optional(executor)
        .await
        .ok()
        .flatten()
    }

    pub async fn update_user_id<'c, E>(
        self,
        executor: E,
        new_user_id: Uuid,
    ) -> Result<Self, ApiError>
    where
        E: Executor<'c, Database = Postgres>,
    {
        Ok(sqlx::query_as(
            r#"
                update "auth_user"
                set user_id = cast($2 as uuid)
                where auth_user_id = cast($1 as uuid)
                returning *
            "#,
        )
        .bind(db_uuid(self.auth_user_id))
        .bind(db_uuid(new_user_id))
        .fetch_one(executor)
        .await?)
    }
}

impl<'r> FromRow<'r, PgRow> for AuthUserRecord {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        use sqlx::Row;

        Ok(Self {
            auth_user_id: parse_uuid(row, "auth_user_id")?,
            user_id: parse_optional_uuid(row, "user_id")?,
            telegram_id: row.try_get("telegram_id").ok(),
            email: row.try_get("email").ok(),
            password_hash: row.try_get("password_hash").ok(),
        })
    }
}

// This is what gets derived from user's access token
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AuthClaims {
    pub user: Option<User>,
    pub exp: usize,
}

// This is what gets derived from user's refresh token
#[derive(Clone, Serialize, Deserialize)]
pub struct RefreshClaims {
    pub auth_user_id: Uuid,
    pub exp: usize,
}
