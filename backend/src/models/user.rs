use crate::ApiContext;
use crate::models::auth::AuthClaims;
use crate::models::{ApiError, parse_optional_timestamp, parse_timestamp, parse_uuid};
#[cfg(feature = "telegram-auth")]
use crate::telegram;
use http::{HeaderMap, header};
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{FromRow, Row, any::AnyRow};
use tonic::metadata::MetadataMap;
use utoipa::ToSchema;
use uuid::Uuid;

pub struct UserRecord {
    pub user_id: Uuid,
    pub telegram_id: i64,
    pub agent_name: Option<String>,
    pub agent_data_id: Option<Uuid>,
    pub rating: i64,
    pub is_admin: bool,
    pub created_at: time::OffsetDateTime,
    pub updated_at: Option<time::OffsetDateTime>,
}

impl<'r> FromRow<'r, AnyRow> for UserRecord {
    fn from_row(row: &'r AnyRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            user_id: parse_uuid(row, "user_id")?,
            telegram_id: row.get("telegram_id"),
            agent_name: row.try_get("agent_name").ok(),
            agent_data_id: parse_uuid(row, "agent_data_id").ok(),
            rating: row.get("rating"),
            is_admin: row.get("is_admin"),
            created_at: parse_timestamp(row, "created_at")?,
            updated_at: parse_optional_timestamp(row, "updated_at").ok().flatten(),
        })
    }
}

#[derive(Serialize, ToSchema)]
pub struct UserResponse {
    pub user_id: Uuid,
    pub telegram_id: i64,
    pub agent_name: Option<String>,
    pub agent_data_id: Option<Uuid>,
    pub is_admin: bool,
    pub rating: i64,
    pub created_at: String,
    pub updated_at: Option<String>,
}

impl From<&UserRecord> for UserResponse {
    fn from(value: &UserRecord) -> Self {
        Self {
            user_id: value.user_id,
            telegram_id: value.telegram_id,
            agent_name: value.agent_name.clone(),
            agent_data_id: value.agent_data_id,
            is_admin: value.is_admin,
            rating: value.rating,
            created_at: value
                .created_at
                .format(&time::format_description::well_known::Rfc3339)
                .unwrap_or_default(),
            updated_at: value.updated_at.and_then(|t| {
                t.format(&time::format_description::well_known::Rfc3339)
                    .ok()
            }),
        }
    }
}

#[derive(Deserialize, ToSchema)]
pub struct CreateUserRequest {
    pub telegram_id: i64,
    pub agent_name: Option<String>,
    pub agent_data: Option<Value>,
    pub is_admin: Option<bool>,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateUserRequest {
    pub telegram_id: Option<i64>,
    pub agent_name: Option<String>,
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
