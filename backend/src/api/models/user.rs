use crate::api::models::{parse_json, parse_optional_timestamp, parse_timestamp, parse_uuid};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{FromRow, Row, any::AnyRow};
use uuid::Uuid;

pub struct UserRecord {
    pub user_id: Uuid,
    pub telegram_id: i64,
    pub agent_name: Option<String>,
    pub agent_data: Value,
    pub is_admin: bool,
    pub created_at: sqlx::types::time::OffsetDateTime,
    pub updated_at: Option<sqlx::types::time::OffsetDateTime>,
}

impl<'r> FromRow<'r, AnyRow> for UserRecord {
    fn from_row(row: &'r AnyRow) -> Result<Self, sqlx::Error> {
        let is_admin: i64 = row.try_get("is_admin")?;
        Ok(Self {
            user_id: parse_uuid(row, "user_id")?,
            telegram_id: row.try_get("telegram_id")?,
            agent_name: row.try_get("agent_name")?,
            agent_data: parse_json(row, "agent_data")?,
            is_admin: if is_admin != 0 {true} else {false},
            created_at: parse_timestamp(row, "created_at")?,
            updated_at: parse_optional_timestamp(row, "updated_at")?,
        })
    }
}

#[derive(Serialize)]
pub struct UserResponse {
    pub user_id: Uuid,
    pub telegram_id: i64,
    pub agent_name: Option<String>,
    pub agent_data: Value,
    pub is_admin: bool,
    pub rating: i64,
    pub created_at: String,
    pub updated_at: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub telegram_id: i64,
    pub agent_name: Option<String>,
    pub agent_data: Option<Value>,
    pub is_admin: Option<bool>,
}

#[derive(Deserialize)]
pub struct UpdateUserRequest {
    pub telegram_id: Option<i64>,
    pub agent_name: Option<String>,
    pub agent_data: Option<Value>,
    pub is_admin: Option<bool>,
}
