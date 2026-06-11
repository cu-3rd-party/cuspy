use crate::api::models::{parse_optional_timestamp, parse_timestamp, parse_uuid};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{any::AnyRow, FromRow, Row};
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
        let is_admin: i64 = row.try_get("is_admin")?;
        Ok(Self {
            user_id: parse_uuid(row, "user_id")?,
            telegram_id: row.try_get("telegram_id")?,
            agent_name: row.try_get("agent_name")?,
            agent_data_id: parse_uuid(row, "agent_data_uuid").ok(),
            rating: row.try_get("rating")?,
            is_admin: if is_admin != 0 { true } else { false },
            created_at: parse_timestamp(row, "created_at")?,
            updated_at: parse_optional_timestamp(row, "updated_at")?,
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
