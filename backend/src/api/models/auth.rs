use crate::api::models::parse_uuid;
use crate::api::models::user::UserResponse;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, any::AnyRow};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub email: Option<String>,
    pub password: Option<String>,
    pub telegram_id: Option<i64>,
    pub agent_name: Option<String>,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: Option<String>,
    pub password: Option<String>,
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

pub struct AuthUserRecord {
    pub auth_user_id: Uuid,
    pub user_id: Uuid,
    pub login_identifier: String,
    #[cfg_attr(feature = "telegram-auth", allow(dead_code))]
    pub password_hash: Option<String>,
}

impl<'r> FromRow<'r, AnyRow> for AuthUserRecord {
    fn from_row(row: &'r AnyRow) -> Result<Self, sqlx::Error> {
        use sqlx::Row;

        Ok(Self {
            auth_user_id: parse_uuid(row, "auth_user_id")?,
            user_id: parse_uuid(row, "user_id")?,
            login_identifier: row.try_get("login_identifier")?,
            password_hash: row.try_get("password_hash")?,
        })
    }
}

// This is what gets derived from user's auth token
#[derive(Clone, Serialize, Deserialize)]
pub struct AuthClaims {
    pub sub: String,
    pub user_id: Uuid,
    pub auth_user_id: Uuid,
    pub is_admin: bool,
    pub exp: usize,
}

// This is what gets derived from user's refresh token
#[derive(Clone, Serialize, Deserialize)]
pub struct RefreshClaims {
    pub sub: String,
    pub user_id: Uuid,
    pub auth_user_id: Uuid,
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
