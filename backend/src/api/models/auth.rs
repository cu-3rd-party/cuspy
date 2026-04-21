use sqlx::FromRow;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::api::models::user::UserResponse;

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub telegram_id: i64,
    pub rating: Option<i64>,
    pub agent_name: Option<String>,
    pub agent_data: Option<Value>,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[cfg(feature = "telegram-auth")]
#[derive(Deserialize)]
pub struct TelegramUser {
    pub id: i64,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub user: UserResponse,
}

#[derive(FromRow)]
pub struct AuthUserRecord {
    pub auth_user_id: Uuid,
    pub user_id: Uuid,
    pub email: String,
    pub password_hash: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AuthClaims {
    pub sub: String,
    pub user_id: Uuid,
    pub auth_user_id: Uuid,
    pub exp: usize,
}

#[derive(Clone)]
pub struct AuthenticatedUser {
    pub user_id: Uuid,
}

#[cfg(feature = "telegram-auth")]
pub struct TelegramInitData {
    pub telegram_user_id: i64,
}