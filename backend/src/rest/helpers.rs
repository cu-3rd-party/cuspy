use std::time::{SystemTime, UNIX_EPOCH};

use crate::models::ApiError;
use crate::models::auth::{AuthClaims, AuthTokenPair, AuthUserRecord, RefreshClaims};
use crate::models::similarity::SimilarityResponse;
use crate::models::user::{User, UserResponse};
use crate::{ApiContext, r#const};
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde_json::Value;
use uuid::Uuid;

pub const DEFAULT_RATING: i64 = 1000;

pub fn format_timestamp(value: time::OffsetDateTime) -> String {
    value.unix_timestamp().to_string()
}

pub fn to_user_response(user: User) -> UserResponse {
    UserResponse {
        user_id: user.user_id,
        username: user.username,
        agent_data: None,
        rating: user.rating,
        is_admin: user.is_admin,
        created_at: format_timestamp(user.created_at),
        updated_at: user.updated_at.map(format_timestamp),
    }
}

pub fn ensure_owner(auth: &User, owner_user_id: Uuid) -> Result<(), ApiError> {
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

pub fn create_token_pair(
    state: &ApiContext,
    auth_user: &AuthUserRecord,
    user: Option<User>,
) -> Result<AuthTokenPair, ApiError> {
    Ok(AuthTokenPair {
        access_token: create_access_token(state, user)?,
        refresh_token: create_refresh_token(state, auth_user)?,
    })
}

fn create_refresh_token(
    state: &ApiContext,
    auth_user: &AuthUserRecord,
) -> Result<String, ApiError> {
    let exp = SystemTime::now()
        .checked_add(r#const::REFRESH_TOKEN_TTL)
        .ok_or(ApiError::Token)?
        .duration_since(UNIX_EPOCH)
        .map_err(|_| ApiError::Token)?
        .as_secs() as usize;

    let claims = RefreshClaims {
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

fn create_access_token(state: &ApiContext, user: Option<User>) -> Result<String, ApiError> {
    let exp = SystemTime::now()
        .checked_add(r#const::AUTH_TOKEN_TTL)
        .ok_or(ApiError::Token)?
        .duration_since(UNIX_EPOCH)
        .map_err(|_| ApiError::Token)?
        .as_secs() as usize;

    let claims = AuthClaims { user, exp };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.jwt_secret.as_bytes()),
    )
    .map_err(|_| ApiError::Token)
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;
    use hmac::{Hmac, Mac};
    use http::{HeaderMap, HeaderValue, header};
    use s3::creds::Credentials;
    use s3::{Bucket, Region};
    use serde_json::json;
    use sha2::{Digest, Sha256};
    use sqlx::postgres::PgPoolOptions;
    use std::sync::Arc;

    fn test_bucket() -> Box<Bucket> {
        Bucket::new(
            "test-bucket",
            Region::Custom {
                region: "us-east-1".into(),
                endpoint: "http://127.0.0.1:9000".into(),
            },
            Credentials {
                access_key: Some("test".into()),
                secret_key: Some("test".into()),
                security_token: None,
                session_token: None,
                expiration: None,
            },
        )
        .expect("test bucket")
        .with_path_style()
    }

    fn test_state() -> ApiContext {
        ApiContext {
            db: PgPoolOptions::new()
                .connect_lazy("postgres://postgres:postgres@127.0.0.1/postgres")
                .expect("lazy pool"),
            bucket: Arc::new(test_bucket()),
            admin_secret: "admin-secret".into(),
            config: crate::config::Config::parse_from([""]),
            jwt_secret: "jwt-secret".into(),
            profile_request_tx: tokio::sync::broadcast::channel(16).0,
            #[cfg(feature = "telegram")]
            telegram_bot_token: "bot-token".into(),
            #[cfg(feature = "telegram")]
            public_webapp_url: "https://example.com".into(),
        }
    }

    #[tokio::test]
    async fn bearer_tokens_round_trip_admin_claims() {
        let state = test_state();
        let auth_user = AuthUserRecord::default();
        let mut user = User::default();
        user.is_admin = true;
        let expected_user_id = user.user_id.clone();
        let token = create_token_pair(&state, &auth_user, Some(user)).expect("token_pair");
        let mut headers = HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", token.access_token)).expect("header"),
        );

        let auth = User::from_headers(&state, &headers);
        assert!(auth.is_ok());
        let auth = auth.expect("authenticated user");
        assert_eq!(auth.user_id, expected_user_id);
        assert!(auth.is_admin);
    }

    #[tokio::test]
    async fn bearer_tokens_round_trip_no_admin_claims() {
        let state = test_state();
        let auth_user = AuthUserRecord::default();
        let user = User::default();
        let expected_user_id = user.user_id.clone();
        let token = create_token_pair(&state, &auth_user, Some(user)).expect("token_pair");
        let mut headers = HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", token.access_token)).expect("header"),
        );

        let auth = User::from_headers(&state, &headers);
        assert!(auth.is_ok());
        let auth = auth.expect("authenticated user");
        assert_eq!(auth.user_id, expected_user_id);
        assert!(!auth.is_admin);
    }

    #[tokio::test]
    async fn admin_secret_fallback_works_without_bearer_token() {
        let state = test_state();
        let mut headers = HeaderMap::new();
        headers.insert(
            header::HeaderName::from_static("x-admin-secret"),
            HeaderValue::from_static("admin-secret"),
        );

        let auth = User::from_headers(&state, &headers);
        assert!(auth.is_ok());
        let auth = auth.expect("authenticated user");
        assert!(auth.is_admin);
        assert_eq!(auth.user_id, Uuid::nil());
    }

    #[tokio::test]
    async fn admin_secret_fallback_works_with_bearer_token() {
        let state = test_state();
        let auth_user = AuthUserRecord::default();
        let mut user = User::default();
        let expected_user_id = user.user_id.clone();
        user.is_admin = false; // explicitly setting not admin for test purposes even though it's supplied through default() impl
        let token = create_token_pair(&state, &auth_user, Some(user)).expect("token_pair");
        let mut headers = HeaderMap::new();
        headers.insert(
            header::HeaderName::from_static("x-admin-secret"),
            HeaderValue::from_static("admin-secret"),
        );
        headers.insert(
            header::AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", token.access_token)).expect("header"),
        );

        let auth = User::from_headers(&state, &headers);
        assert!(auth.is_ok());
        let auth = auth.expect("authenticated user");
        assert!(auth.is_admin);
        assert_eq!(auth.user_id, expected_user_id);
    }
}
