use crate::models::parse_uuid;
use crate::models::user::{User, UserResponse};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, postgres::PgRow};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize, ToSchema)]
pub struct RegisterRequest {
    pub email: Option<String>,
    pub password: Option<String>,
    pub telegram_id: Option<i64>,
    pub username: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct LoginRequest {
    pub email: Option<String>,
    pub password: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct TelegramInitDataRequest {
    pub init_data: String,
}

#[derive(Serialize, ToSchema)]
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

impl<'r> FromRow<'r, PgRow> for AuthUserRecord {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        use sqlx::Row;

        Ok(Self {
            auth_user_id: parse_uuid(row, "auth_user_id")?,
            user_id: parse_uuid(row, "user_id")?,
            login_identifier: row.get("login_identifier"),
            password_hash: row.try_get("password_hash").ok(),
        })
    }
}

// This is what gets derived from user's access token
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AuthClaims {
    pub sub: String,
    pub user: Option<User>,
    pub exp: usize,
}

// This is what gets derived from user's refresh token
#[derive(Clone, Serialize, Deserialize)]
pub struct RefreshClaims {
    pub sub: String,
    pub auth_user_id: Uuid,
    pub exp: usize,
}
