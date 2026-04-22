use crate::api::models::user::UserResponse;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct RegisterRequest {
    #[cfg(not(feature = "telegram-auth"))]
    pub email: String,
    #[cfg(not(feature = "telegram-auth"))]
    pub password: String,
    #[cfg(not(feature = "telegram-auth"))]
    pub telegram_id: i64,
    pub agent_name: Option<String>,
    pub agent_data: Option<Value>,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    #[cfg(not(feature = "telegram-auth"))]
    pub email: String,
    #[cfg(not(feature = "telegram-auth"))]
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
    pub login_identifier: String,
    #[cfg_attr(feature = "telegram-auth", allow(dead_code))]
    pub password_hash: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AuthClaims {
    pub sub: String,
    pub user_id: Uuid,
    pub auth_user_id: Uuid,
    pub is_admin: bool,
    pub exp: usize,
}

#[derive(Clone)]
pub struct AuthenticatedUser {
    pub user_id: Uuid,
    pub is_admin: bool,
    #[cfg_attr(feature = "telegram-auth", allow(dead_code))]
    #[cfg(feature = "telegram-auth")]
    pub telegram_user_id: i64,
}

#[cfg(feature = "telegram-auth")]
pub struct TelegramInitData {
    pub telegram_user_id: i64,
}
