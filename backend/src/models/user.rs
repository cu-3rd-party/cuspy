use crate::ApiContext;
use crate::models::auth::AuthClaims;
use crate::models::{ApiError, parse_optional_timestamp, parse_timestamp, parse_uuid, db_uuid, db_optional_uuid};
use crate::models::agent_data::AgentData;
use http::{HeaderMap, header};
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row, postgres::PgRow, Acquire, Postgres};
use tonic::metadata::MetadataMap;
use utoipa::ToSchema;
use uuid::Uuid;
use crate::rest::helpers::format_timestamp;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub user_id: Uuid,
    pub username: Option<String>,
    pub agent_data_id: Option<Uuid>,
    pub rating: i64,
    pub is_admin: bool,
    pub created_at: time::OffsetDateTime,
    pub updated_at: Option<time::OffsetDateTime>,
}
#[derive(Debug, Serialize, ToSchema, Default)]
pub struct UserResponse {
    pub user_id: Uuid,
    pub username: Option<String>,
    pub agent_data: Option<AgentData>,
    pub rating: i64,
    pub is_admin: bool,
    pub created_at: String,
    pub updated_at: Option<String>,
}

impl User {
    pub async fn create<'c, A>(
        executor: A,
        username: String,
        is_admin: bool,
        agent_data: Option<AgentData>,
    ) -> Result<Self, ApiError>
    where
        A: Acquire<'c, Database = Postgres>
    {
        let mut executor = executor.acquire().await?;
        let user: User = sqlx::query_as(
        r#"
                insert into "user" (username, is_admin, agent_data_id)
                values ($1, $2, cast($3 as uuid))
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
            .bind(db_optional_uuid(agent_data.map(|d| d.agent_data_id)))
            .fetch_one(&mut *executor)
            .await?;
        Ok(user)
    }

    pub async fn update<'c, A>(
        &mut self,
        executor: A,
    ) -> Result<(), ApiError>
    where
        A: Acquire<'c, Database = Postgres>
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
            .bind(self.username.clone())
            .bind(db_optional_uuid(self.agent_data_id.clone()))
            .bind(self.is_admin.clone())
            .fetch_one(&mut *executor)
            .await?;
        Ok(())
    }

    pub async fn get_by_id<'c, A>(
        executor: A,
        user_id: Uuid,
    ) -> Option<Self>
    where
        A: Acquire<'c, Database = Postgres>
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
        A: Acquire<'c, Database = Postgres>
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

    pub async fn into_response<'c, A>(
        self,
        executor: A,
    ) -> Result<UserResponse, ApiError>
    where
        A: Acquire<'c, Database = Postgres>
    {
        let mut executor = executor.acquire().await?;
        let agent_data = match self.agent_data_id {
            Some(id) => AgentData::get_by_id(&mut *executor, id).await,
            None => None,
        };

        Ok(UserResponse {
            user_id: self.user_id,
            username: self.username,
            agent_data,
            rating: self.rating,
            is_admin: self.is_admin,
            created_at: format_timestamp(self.created_at),
            updated_at: self.updated_at.map(format_timestamp),
        })
    }
}

impl<'r> FromRow<'r, PgRow> for User {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
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
pub struct UserRequest {
    pub telegram_id: i64,
    pub username: Option<String>,
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
                username: None,
                agent_data_id: None,
                rating: 0,
                is_admin: true,
                created_at: time::OffsetDateTime::UNIX_EPOCH,
                updated_at: None,
            });
        }

        let auth_token = auth_token.ok_or(ApiError::BadRequest(
            "no authorization header supplied".to_string(),
        ))?;

        let decoded = decode::<AuthClaims>(
            &auth_token,
            &DecodingKey::from_secret(state.jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| ApiError::Unauthorized)?;

        decoded.claims.user.ok_or(ApiError::Unauthorized)
    }
}
