use std::time::{SystemTime, UNIX_EPOCH};

use crate::api::r#const::{AUTH_HEADER_PREFIX, USER_TOKEN_TTL};
use http::{header, HeaderMap};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use uuid::Uuid;
use serde_json::{json, Value};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
#[cfg(feature = "telegram-auth")]
use hmac::Hmac;
#[cfg(feature = "telegram-auth")]
use hmac::Mac;
#[cfg(feature = "telegram-auth")]
use hmac::digest::Digest;
#[cfg(feature = "telegram-auth")]
use sha2::Sha256;
#[cfg(feature = "telegram-auth")]
use url::form_urlencoded;
use crate::api::models::ApiError;
use crate::api::models::auth::{AuthClaims, AuthUserRecord, AuthenticatedUser};
#[cfg(feature = "telegram-auth")]
use crate::api::models::auth::{TelegramInitData, TelegramUser};
use crate::api::models::profile::{ProfileCreationRequestRecord, ProfileCreationRequestResponse};
use crate::api::models::similarity::SimilarityResponse;
use crate::api::models::user::{UserRecord, UserResponse};
use crate::AppState;

fn format_timestamp(value: sqlx::types::time::OffsetDateTime) -> String {
    value.unix_timestamp().to_string()
}

pub fn to_user_response(record: UserRecord) -> UserResponse {
    UserResponse {
        user_id: record.user_id,
        telegram_id: record.telegram_id,
        rating: record.rating,
        agent_name: record.agent_name,
        agent_data: record.agent_data,
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
        requested_profile_data: record.requested_profile_data,
        status: record.status,
        reviewer_note: record.reviewer_note,
        reviewed_at: record.reviewed_at.map(format_timestamp),
        created_at: format_timestamp(record.created_at),
        updated_at: format_timestamp(record.updated_at),
    }
}

pub fn require_admin(headers: &HeaderMap, state: &AppState) -> Result<(), ApiError> {
    let Some(value) = headers.get(header::HeaderName::from_static("x-admin-secret")) else {
        return Err(ApiError::Unauthorized);
    };

    let Ok(secret) = value.to_str() else {
        return Err(ApiError::Unauthorized);
    };

    if secret != state.admin_secret {
        return Err(ApiError::Unauthorized);
    }

    Ok(())
}

#[cfg(feature = "telegram-auth")]
pub fn verify_telegram_init_data(headers: &HeaderMap, state: &AppState) -> Result<TelegramInitData, ApiError> {
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
    let mut mac = HmacSha256::new_from_slice(secret.as_slice()).map_err(|_| ApiError::Unauthorized)?;
    mac.update(data_check_string.as_bytes());
    let expected_hash = hex::encode(mac.finalize().into_bytes());

    let provided_hash = hash.ok_or(ApiError::Unauthorized)?;
    if expected_hash != provided_hash {
        return Err(ApiError::Unauthorized);
    }

    let user_json = user_json.ok_or(ApiError::Unauthorized)?;
    let telegram_user: TelegramUser = serde_json::from_str(&user_json)
        .map_err(|_| ApiError::Unauthorized)?;

    Ok(TelegramInitData {
        telegram_user_id: telegram_user.id,
    })
}

#[cfg(not(feature = "telegram-auth"))]
#[allow(dead_code)]
fn verify_telegram_init_data(_headers: &HeaderMap, _state: &AppState) -> Result<(), ApiError> {
    Ok(())
}

pub fn require_bearer_token(headers: &HeaderMap, state: &AppState) -> Result<AuthenticatedUser, ApiError> {
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
    })
}

pub fn ensure_owner(auth: &AuthenticatedUser, owner_user_id: Uuid) -> Result<(), ApiError> {
    if auth.user_id != owner_user_id {
        return Err(ApiError::Forbidden);
    }

    Ok(())
}

pub fn normalize_profile_data(value: Option<Value>) -> Result<Value, ApiError> {
    match value {
        None => Ok(json!({})),
        Some(Value::Object(map)) => Ok(Value::Object(map)),
        Some(_) => Err(ApiError::BadRequest("profile data must be a JSON object".into())),
    }
}

pub fn compare_profile_similarity(left: &Value, right: &Value) -> Result<SimilarityResponse, ApiError> {
    let Value::Object(left_map) = left else {
        return Err(ApiError::BadRequest("left profile must be a JSON object".into()));
    };
    let Value::Object(right_map) = right else {
        return Err(ApiError::BadRequest("right profile must be a JSON object".into()));
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

    let union_count = matching_keys.len() + differing_keys.len() + left_only_keys.len() + right_only_keys.len();
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

pub fn hash_password(password: &str) -> Result<String, ApiError> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| ApiError::PasswordHash)?;
    Ok(hash.to_string())
}

pub fn verify_password(hash: &str, password: &str) -> Result<(), ApiError> {
    let parsed_hash = PasswordHash::new(hash).map_err(|_| ApiError::PasswordHash)?;
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|_| ApiError::Unauthorized)
}

pub fn create_access_token(state: &AppState, auth_user: &AuthUserRecord) -> Result<String, ApiError> {
    let exp = SystemTime::now()
        .checked_add(USER_TOKEN_TTL)
        .ok_or(ApiError::Token)?
        .duration_since(UNIX_EPOCH)
        .map_err(|_| ApiError::Token)?
        .as_secs() as usize;

    let claims = AuthClaims {
        sub: auth_user.email.clone(),
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
