use crate::ApiContext;
use crate::api::models::ApiError;
use crate::api::models::auth::AuthClaims;
use http::request::Parts;
use http::{HeaderMap, header};
use jsonwebtoken::{DecodingKey, Validation, decode};
use uuid::Uuid;

#[cfg(feature = "telegram-auth")]
use crate::telegram;

pub const AUTH_HEADER_PREFIX: &str = "Bearer ";

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct User {
    pub user_id: Uuid,
    pub is_admin: bool,
    #[cfg(feature = "telegram-auth")]
    pub tg: telegram::TelegramInitData,
}

pub struct AuthUser(pub User);
pub struct MaybeAuthUser(pub Option<User>);
pub struct AdminUser(pub User);

impl axum::extract::FromRequestParts<ApiContext> for AuthUser {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &ApiContext,
    ) -> Result<Self, Self::Rejection> {
        Ok(Self(User::from_headers(&state, &parts.headers)?))
    }
}

impl axum::extract::FromRequestParts<ApiContext> for MaybeAuthUser {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &ApiContext,
    ) -> Result<Self, Self::Rejection> {
        Ok(Self(User::from_headers(&state, &parts.headers).ok()))
    }
}

impl axum::extract::FromRequestParts<ApiContext> for AdminUser {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &ApiContext,
    ) -> Result<Self, Self::Rejection> {
        let user = User::from_headers(&state, &parts.headers)?;

        if !user.is_admin {
            return Err(ApiError::Unauthorized);
        }
        Ok(Self(user))
    }
}

impl User {
    pub fn from_headers(state: &ApiContext, header_map: &HeaderMap) -> Result<Self, ApiError> {
        let has_valid_admin_header = header_map
            .get("x-admin-secret")
            .or_else(|| header_map.get("Admin"))
            .and_then(|s| s.to_str().ok())
            .map(|header| header == state.admin_secret)
            .unwrap_or(false);

        let auth_token = header_map
            .get(header::AUTHORIZATION)
            .map(|value| value.to_str().map_err(|_| ApiError::Unauthorized))
            .transpose()?
            .map(|value| {
                value
                    .strip_prefix(AUTH_HEADER_PREFIX)
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
        let telegram_init_data = header_map
            .get("x-telegram-init-data")
            .ok_or(ApiError::Unauthorized)?
            .to_str()
            .map_err(|_| ApiError::Unauthorized)?;

        let decoded = decode::<AuthClaims>(
            auth_token,
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
                telegram_init_data,
            )
            .ok_or(ApiError::Unauthorized)?,
        })
    }
}
