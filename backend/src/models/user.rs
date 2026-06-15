use crate::ApiContext;
use crate::models::auth::AuthClaims;
use crate::models::{ApiError, parse_optional_timestamp, parse_timestamp, parse_uuid, db_uuid, db_optional_uuid};
#[cfg(feature = "telegram-auth")]
use crate::telegram;
use http::{HeaderMap, header};
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{FromRow, Row, any::AnyRow, Any, Acquire};
use tonic::metadata::MetadataMap;
use utoipa::ToSchema;
use uuid::Uuid;
use crate::models::agent_data::{AgentData, AgentDataMetadata};
use crate::models::resource::Resource;

pub struct UserRecord {
    pub user_id: Uuid,
    pub username: Option<String>,
    pub agent_data_id: Option<Uuid>,
    pub rating: i64,
    pub is_admin: bool,
    pub created_at: time::OffsetDateTime,
    pub updated_at: Option<time::OffsetDateTime>,
}

#[derive(Serialize, ToSchema)]
pub struct UserResponse {
    pub user_id: Uuid,
    pub username: Option<String>,
    pub agent_data: Option<AgentData>,
    pub is_admin: bool,
    pub rating: i64,
    pub created_at: String,
    pub updated_at: Option<String>,
}

impl UserRecord {
    pub async fn create<'c, A>(
        executor: A,
        username: String,
        is_admin: bool,
        agent_data: Option<AgentData>,
    ) -> Result<Self, ApiError>
    where
        A: Acquire<'c, Database = Any>
    {
        let mut executor = executor.acquire().await?;
        let mut user: UserRecord = sqlx::query_as(
        r#"
                insert into "user" (username, is_admin)
                values ($1, $2)
                returning
                    cast(user_id as text) as user_id,
                    username,
                    cast(agent_data_id as text) as agent_data_id,
                    rating,
                    is_admin,
                    cast(created_at as text) as created_at,
                    cast(updated_at as text) as updated_at
            "#,
        )
            .bind(username)
            .bind(is_admin)
            .fetch_one(&mut *executor)
            .await?;
        if let Some(agent_data) = agent_data {
            user = sqlx::query_as(
                r#"
                    update "user"
                    set agent_data_id = cast($1 as uuid)
                    where user_id = cast($2 as uuid)
                    returning
                        cast(user_id as text) as user_id,
                        username,
                        cast(agent_data_id as text) as agent_data_id,
                        rating,
                        is_admin,
                        cast(created_at as text) as created_at,
                        cast(updated_at as text) as updated_at
                "#
            )
                .bind(db_uuid(agent_data.agent_data_id))
                .bind(db_uuid(user.user_id))
                .fetch_one(&mut *executor)
                .await?;
        }
        Ok(user)
    }

    pub async fn update<'c, A>(
        &mut self,
        executor: A,
    ) -> Result<(), ApiError>
    where
        A: Acquire<'c, Database = Any>
    {
        let mut executor = executor.acquire().await?;
        *self = sqlx::query_as(
            r#"
                update "user"
                set
                    username = $2,
                    agent_data_id = cast($3 as uuid),
                    is_admin = $4
                where user_id = cast($1 as uuid)
                returning
                    cast(user_id as text) as user_id,
                    username,
                    cast(agent_data_id as text) as agent_data_id,
                    rating,
                    is_admin,
                    cast(created_at as text) as created_at,
                    cast(updated_at as text) as updated_at
            "#
        )
            .bind(db_uuid(self.user_id))
            .bind(self.username)
            .bind(db_optional_uuid(self.agent_data_id))
            .bind(self.is_admin)
            .fetch_one(&mut *executor)
            .await?;
        Ok(())
    }

    pub async fn get_by_id<'c, A>(
        executor: A,
        user_id: Uuid,
    ) -> Option<Self>
    where
        A: Acquire<'c, Database = Any>
    {
        let mut executor = executor.acquire().await.ok()?;
        sqlx::query_as(
            r#"
                select * from "user" where user_id = cast($1 as uuid)
            "#
        )
            .bind(db_uuid(user_id))
            .fetch_optional(&mut *executor)
            .await
            .ok().flatten()
    }

    pub async fn get_by_username<'c, A>(
        executor: A,
        username: String,
    ) -> Option<Self>
    where
        A: Acquire<'c, Database = Any>
    {
        let mut executor = executor.acquire().await.ok()?;
        sqlx::query_as(
            r#"
                select * from "user" where username = $1
            "#
        )
            .bind(username)
            .fetch_optional(&mut *executor)
            .await
            .ok().flatten()
    }
}

impl<'r> FromRow<'r, AnyRow> for UserRecord {
    fn from_row(row: &'r AnyRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            user_id: parse_uuid(row, "user_id")?,
            username: row.try_get("username").ok(),
            agent_data_id: parse_uuid(row, "agent_data_id").ok(),
            rating: row.get("rating"),
            is_admin: row.get("is_admin"),
            created_at: parse_timestamp(row, "created_at")?,
            updated_at: parse_optional_timestamp(row, "updated_at").ok().flatten(),
        })
    }
}

#[derive(Deserialize, ToSchema)]
pub struct CreateUserRequest {
    pub telegram_id: i64,
    pub username: Option<String>,
    pub agent_data: Option<Value>,
    pub is_admin: Option<bool>,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateUserRequest {
    pub telegram_id: Option<i64>,
    pub username: Option<String>,
    pub agent_data: Option<Value>,
    pub is_admin: Option<bool>,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct User {
    pub user_id: Uuid,
    pub is_admin: bool,
    #[cfg(feature = "telegram-auth")]
    pub tg: telegram::TelegramInitData,
}

impl User {
    pub fn from_headers(state: &ApiContext, header_map: &HeaderMap) -> Result<Self, ApiError> {
        Self::from_lookup(state, |name| {
            let option = header_map
                .get(name)
                .and_then(|value| value.to_str().ok())
                .map(str::to_owned);
            option
        })
    }

    pub fn from_metadata(state: &ApiContext, metadata: &MetadataMap) -> Result<Self, ApiError> {
        Self::from_lookup(state, |name| {
            metadata
                .get(name)
                .and_then(|value| value.to_str().ok())
                .map(str::to_owned)
        })
    }

    fn from_lookup(
        state: &ApiContext,
        get: impl Fn(&str) -> Option<String>,
    ) -> Result<Self, ApiError> {
        let has_valid_admin_header = get("x-admin-secret")
            .or_else(|| get("admin"))
            .map(|value| value == state.admin_secret)
            .unwrap_or(false);

        let auth_token = get(header::AUTHORIZATION.as_str())
            .map(|value| {
                value
                    .strip_prefix(crate::r#const::AUTH_HEADER_PREFIX)
                    .map(str::to_owned)
                    .ok_or(ApiError::Unauthorized)
            })
            .transpose()?;

        if auth_token.is_none() && has_valid_admin_header {
            return Ok(Self {
                user_id: Uuid::nil(),
                is_admin: true,
                #[cfg(feature = "telegram-auth")]
                tg: telegram::TelegramInitData {
                    user: telegram::TelegramUser { id: 0 },
                },
            });
        }

        let auth_token = auth_token.ok_or(ApiError::BadRequest(
            "no authorization header supplied".to_string(),
        ))?;

        #[cfg(feature = "telegram-auth")]
        let telegram_init_data = get("x-telegram-init-data").ok_or(ApiError::Unauthorized)?;

        let decoded = decode::<AuthClaims>(
            &auth_token,
            &DecodingKey::from_secret(state.jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| ApiError::Unauthorized)?;

        Ok(Self {
            user_id: decoded.claims.user_id,
            is_admin: decoded.claims.is_admin || has_valid_admin_header,
            #[cfg(feature = "telegram-auth")]
            tg: telegram::TelegramInitData::from_header(
                &state.telegram_bot_token,
                &telegram_init_data,
            )
            .ok_or(ApiError::Unauthorized)?,
        })
    }
}
