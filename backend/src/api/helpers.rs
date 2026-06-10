use std::time::{SystemTime, UNIX_EPOCH};

use crate::AppState;
use crate::api::r#const::{AUTH_HEADER_PREFIX, AUTH_TOKEN_TTL, REFRESH_TOKEN_TTL};
use crate::api::models::ApiError;
use crate::api::models::auth::{AuthClaims, AuthUserRecord, AuthenticatedUser, RefreshClaims};
#[cfg(feature = "telegram-auth")]
use crate::api::models::auth::{TelegramInitData, TelegramUser};
use crate::api::models::db_uuid;
use crate::api::models::profile::{ProfileCreationRequestRecord, ProfileCreationRequestResponse};
use crate::api::models::similarity::SimilarityResponse;
use crate::api::models::user::{UserRecord, UserResponse};
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
#[cfg(feature = "telegram-auth")]
use hmac::Hmac;
#[cfg(feature = "telegram-auth")]
use hmac::Mac;
#[cfg(feature = "telegram-auth")]
use hmac::digest::Digest;
use http::{HeaderMap, header};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde_json::{Value, json};
#[cfg(feature = "telegram-auth")]
use sha2::Sha256;
use sqlx::Row;
#[cfg(feature = "telegram-auth")]
use url::form_urlencoded;
use uuid::Uuid;

pub const DEFAULT_RATING: i64 = 1000;

fn format_timestamp(value: sqlx::types::time::OffsetDateTime) -> String {
    value.unix_timestamp().to_string()
}

pub fn to_user_response(record: UserRecord) -> UserResponse {
    UserResponse {
        user_id: record.user_id,
        telegram_id: record.telegram_id,
        agent_name: record.agent_name,
        agent_data_id: record.agent_data_id,
        is_admin: record.is_admin,
        rating: record.rating,
        created_at: format_timestamp(record.created_at),
        updated_at: record.updated_at.map(format_timestamp),
    }
}

pub fn to_profile_creation_request_response(
    record: ProfileCreationRequestRecord,
) -> ProfileCreationRequestResponse {
    ProfileCreationRequestResponse {
        profile_creation_request_id: record.profile_creation_request_id,
        user_id: record.user_id,
        requested_profile_data_id: record.requested_profile_data_id,
        status: record.status,
        reviewer_note: record.reviewer_note,
        reviewed_at: record.reviewed_at.map(format_timestamp),
        created_at: format_timestamp(record.created_at),
        updated_at: format_timestamp(record.updated_at),
    }
}

pub fn require_admin(headers: &HeaderMap, state: &AppState) -> Result<AuthenticatedUser, ApiError> {
    if let Ok(auth) = require_bearer_token(headers, state) {
        if auth.is_admin {
            return Ok(auth);
        }

        return Err(ApiError::Forbidden);
    }

    if let Some(value) = headers.get(header::HeaderName::from_static("x-admin-secret")) {
        let secret = value.to_str().map_err(|_| ApiError::Unauthorized)?;
        if secret == state.admin_secret {
            return Ok(AuthenticatedUser {
                user_id: Uuid::nil(),
                is_admin: true,
                #[cfg(feature = "telegram-auth")]
                telegram_user_id: 0,
            });
        }
    }

    Err(ApiError::Unauthorized)
}

#[cfg(feature = "telegram-auth")]
pub fn verify_telegram_init_data(
    headers: &HeaderMap,
    state: &AppState,
) -> Result<TelegramInitData, ApiError> {
    type HmacSha256 = Hmac<Sha256>;

    let Some(value) = headers.get(header::HeaderName::from_static("x-telegram-init-data")) else {
        return Err(ApiError::Unauthorized);
    };

    let init_data = value.to_str().map_err(|_| ApiError::Unauthorized)?;
    let mut hash = None;
    let mut user_json = None;
    let mut data_pairs = Vec::new();

    for (key, value) in form_urlencoded::parse(init_data.as_bytes()) {
        if key == "hash" {
            hash = Some(value.into_owned());
        } else {
            if key == "user" {
                user_json = Some(value.clone().into_owned());
            }
            data_pairs.push(format!("{key}={value}"));
        }
    }

    data_pairs.sort();
    let data_check_string = data_pairs.join("\n");
    let secret = Sha256::digest(state.telegram_bot_token.as_bytes());
    let mut mac =
        HmacSha256::new_from_slice(secret.as_slice()).map_err(|_| ApiError::Unauthorized)?;
    mac.update(data_check_string.as_bytes());
    let expected_hash = hex::encode(mac.finalize().into_bytes());

    let provided_hash = hash.ok_or(ApiError::Unauthorized)?;
    if expected_hash != provided_hash {
        return Err(ApiError::Unauthorized);
    }

    let user_json = user_json.ok_or(ApiError::Unauthorized)?;
    let telegram_user: TelegramUser =
        serde_json::from_str(&user_json).map_err(|_| ApiError::Unauthorized)?;

    Ok(TelegramInitData {
        telegram_user_id: telegram_user.id,
    })
}

#[cfg(not(feature = "telegram-auth"))]
#[allow(dead_code)]
fn verify_telegram_init_data(_headers: &HeaderMap, _state: &AppState) -> Result<(), ApiError> {
    Ok(())
}

pub fn optional_telegram_user_id(
    headers: &HeaderMap,
    state: &AppState,
) -> Result<Option<i64>, ApiError> {
    #[cfg(feature = "telegram-auth")]
    {
        if !state.telegram_auth_enabled() {
            return Ok(None);
        }

        return Ok(Some(
            verify_telegram_init_data(headers, state)?.telegram_user_id,
        ));
    }

    #[cfg(not(feature = "telegram-auth"))]
    {
        let _ = headers;
        let _ = state;
        Ok(None)
    }
}

pub fn require_bearer_token(
    headers: &HeaderMap,
    state: &AppState,
) -> Result<AuthenticatedUser, ApiError> {
    let Some(value) = headers.get(header::AUTHORIZATION) else {
        return Err(ApiError::Unauthorized);
    };

    let token = value.to_str().map_err(|_| ApiError::Unauthorized)?;
    let token = token
        .strip_prefix(AUTH_HEADER_PREFIX)
        .ok_or(ApiError::Unauthorized)?;

    let decoded = decode::<AuthClaims>(
        token,
        &DecodingKey::from_secret(state.jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| ApiError::Unauthorized)?;

    Ok(AuthenticatedUser {
        user_id: decoded.claims.user_id,
        is_admin: decoded.claims.is_admin,
        #[cfg(feature = "telegram-auth")]
        telegram_user_id: decoded
            .claims
            .sub
            .parse()
            .map_err(|_| ApiError::Unauthorized)?,
    })
}

pub fn ensure_owner(auth: &AuthenticatedUser, owner_user_id: Uuid) -> Result<(), ApiError> {
    if auth.user_id != owner_user_id {
        return Err(ApiError::Forbidden);
    }

    Ok(())
}

pub fn compare_profile_similarity(
    left: &Value,
    right: &Value,
) -> Result<SimilarityResponse, ApiError> {
    let Value::Object(left_map) = left else {
        return Err(ApiError::BadRequest(
            "left profile must be a JSON object".into(),
        ));
    };
    let Value::Object(right_map) = right else {
        return Err(ApiError::BadRequest(
            "right profile must be a JSON object".into(),
        ));
    };

    let mut matching_keys = Vec::new();
    let mut differing_keys = Vec::new();
    let mut left_only_keys = Vec::new();
    let mut right_only_keys = Vec::new();

    for (key, left_value) in left_map {
        match right_map.get(key) {
            Some(right_value) if right_value == left_value => matching_keys.push(key.clone()),
            Some(_) => differing_keys.push(key.clone()),
            None => left_only_keys.push(key.clone()),
        }
    }

    for key in right_map.keys() {
        if !left_map.contains_key(key) {
            right_only_keys.push(key.clone());
        }
    }

    matching_keys.sort();
    differing_keys.sort();
    left_only_keys.sort();
    right_only_keys.sort();

    let union_count =
        matching_keys.len() + differing_keys.len() + left_only_keys.len() + right_only_keys.len();
    let similarity_score = if union_count == 0 {
        1.0
    } else {
        matching_keys.len() as f64 / union_count as f64
    };

    Ok(SimilarityResponse {
        similarity_score,
        matching_keys,
        differing_keys,
        left_only_keys,
        right_only_keys,
    })
}

#[cfg_attr(feature = "telegram-auth", allow(dead_code))]
pub fn hash_password(password: &str) -> Result<String, ApiError> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| ApiError::PasswordHash)?;
    Ok(hash.to_string())
}

#[cfg_attr(feature = "telegram-auth", allow(dead_code))]
pub fn verify_password(hash: &str, password: &str) -> Result<(), ApiError> {
    let parsed_hash = PasswordHash::new(hash).map_err(|_| ApiError::PasswordHash)?;
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|_| ApiError::Unauthorized)
}

pub fn create_access_token(
    state: &AppState,
    auth_user: &AuthUserRecord,
    is_admin: bool,
) -> Result<String, ApiError> {
    let exp = SystemTime::now()
        .checked_add(AUTH_TOKEN_TTL)
        .ok_or(ApiError::Token)?
        .duration_since(UNIX_EPOCH)
        .map_err(|_| ApiError::Token)?
        .as_secs() as usize;

    let claims = AuthClaims {
        sub: auth_user.login_identifier.clone(),
        user_id: auth_user.user_id,
        auth_user_id: auth_user.auth_user_id,
        is_admin,
        exp,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.jwt_secret.as_bytes()),
    )
    .map_err(|_| ApiError::Token)
}

pub fn create_refresh_token(
    state: &AppState,
    auth_user: &AuthUserRecord,
) -> Result<String, ApiError> {
    let exp = SystemTime::now()
        .checked_add(REFRESH_TOKEN_TTL)
        .ok_or(ApiError::Token)?
        .duration_since(UNIX_EPOCH)
        .map_err(|_| ApiError::Token)?
        .as_secs() as usize;

    let claims = RefreshClaims {
        sub: auth_user.login_identifier.clone(),
        user_id: auth_user.user_id,
        auth_user_id: auth_user.auth_user_id,
        exp,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.jwt_secret.as_bytes()),
    )
    .map_err(|_| ApiError::Token)
}

pub async fn fetch_user(db: &sqlx::AnyPool, user_id: Uuid) -> Result<UserRecord, ApiError> {
    Ok(sqlx::query_as(
        r#"
            select *
            from "user"
            where user_id = $1
            limit 1
            "#,
    )
    .bind(db_uuid(user_id))
    .fetch_one(db)
    .await?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use http::HeaderValue;
    use serde_json::json;
    use sqlx::any::AnyPoolOptions;

    fn test_state() -> AppState {
        AppState {
            db: AnyPoolOptions::new()
                .connect_lazy("postgres://postgres:postgres@127.0.0.1/postgres")
                .expect("lazy pool"),
            admin_secret: "admin-secret".into(),
            jwt_secret: "jwt-secret".into(),
            #[cfg(feature = "telegram-auth")]
            telegram_bot_token: Some("bot-token".into()),
            #[cfg(feature = "telegram-auth")]
            public_webapp_url: Some("https://example.com".into()),
        }
    }

    #[test]
    fn compare_profile_similarity_tracks_matching_keys() {
        let response = compare_profile_similarity(
            &json!({ "city": "Kyiv", "course": 3, "track": "backend" }),
            &json!({ "city": "Kyiv", "course": 4, "squad": "red" }),
        )
        .expect("similarity response");

        assert_eq!(response.matching_keys, vec!["city"]);
        assert_eq!(response.differing_keys, vec!["course"]);
        assert_eq!(response.left_only_keys, vec!["track"]);
        assert_eq!(response.right_only_keys, vec!["squad"]);
        assert!((response.similarity_score - 0.25).abs() < f64::EPSILON);
    }

    #[tokio::test]
    async fn bearer_tokens_round_trip_admin_claims() {
        let state = test_state();
        let auth_user = AuthUserRecord {
            auth_user_id: Uuid::now_v7(),
            user_id: Uuid::now_v7(),
            login_identifier: {
                #[cfg(feature = "telegram-auth")]
                {
                    "12345".into()
                }
                #[cfg(not(feature = "telegram-auth"))]
                {
                    "agent@example.com".into()
                }
            },
            password_hash: Some("hash".into()),
        };
        let token = create_access_token(&state, &auth_user, true).expect("token");
        let mut headers = HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {token}")).expect("header"),
        );

        let auth = require_bearer_token(&headers, &state).expect("auth user");
        assert_eq!(auth.user_id, auth_user.user_id);
        assert!(auth.is_admin);
    }

    #[tokio::test]
    async fn admin_secret_fallback_works_without_bearer_token() {
        let state = test_state();
        let mut headers = HeaderMap::new();
        headers.insert(
            header::HeaderName::from_static("x-admin-secret"),
            HeaderValue::from_static("admin-secret"),
        );

        let auth = require_admin(&headers, &state).expect("admin auth");
        assert!(auth.is_admin);
        assert_eq!(auth.user_id, Uuid::nil());
    }
}
