use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub struct UserRecord {
    pub user_id: Uuid,
    pub telegram_id: i64,
    pub agent_name: Option<String>,
    pub agent_data: Value,
    pub is_admin: bool,
    pub created_at: sqlx::types::time::OffsetDateTime,
    pub updated_at: Option<sqlx::types::time::OffsetDateTime>,
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
