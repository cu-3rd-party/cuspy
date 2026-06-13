use std::time::{SystemTime, UNIX_EPOCH};

use crate::models::ApiError;
use crate::models::auth::{AuthClaims, AuthUserRecord, RefreshClaims};
use crate::models::db_uuid;
use crate::models::profile::{ProfileRequestRecord, ProfileRequestResponse};
use crate::models::similarity::SimilarityResponse;
use crate::models::user::User;
use crate::models::user::{UserRecord, UserResponse};
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

pub fn to_profile_request_response(record: ProfileRequestRecord) -> ProfileRequestResponse {
    ProfileRequestResponse {
        profile_request_id: record.profile_request_id,
        user_id: record.user_id,
        requested_profile_data_id: record.requested_profile_data_id,
        status: record.status,
        reviewer_note: record.reviewer_note,
        reviewed_at: record.reviewed_at.map(format_timestamp),
        created_at: format_timestamp(record.created_at),
        updated_at: format_timestamp(record.updated_at),
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
    state: &ApiContext,
    auth_user: &AuthUserRecord,
    is_admin: bool,
) -> Result<String, ApiError> {
    let exp = SystemTime::now()
        .checked_add(r#const::AUTH_TOKEN_TTL)
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
            select
                cast(user_id as text) as user_id,
                telegram_id,
                agent_name,
                cast(agent_data_id as text) as agent_data_id,
                rating,
                is_admin,
                cast(created_at as text) as created_at,
                cast(updated_at as text) as updated_at
            from "user"
            where user_id = cast($1 as uuid)
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
    use clap::Parser;
    #[cfg(feature = "telegram-auth")]
    use hmac::{Hmac, Mac};
    use http::{HeaderMap, HeaderValue, header};
    use s3::creds::Credentials;
    use s3::{Bucket, Region};
    use serde_json::json;
    #[cfg(feature = "telegram-auth")]
    use sha2::{Digest, Sha256};
    use sqlx::any::AnyPoolOptions;

    #[cfg(feature = "telegram-auth")]
    fn telegram_init_data(user_id: i64) -> String {
        type HmacSha256 = Hmac<Sha256>;

        let user = json!({
            "id": user_id,
            "first_name": "Test",
            "username": format!("user_{user_id}")
        })
        .to_string();

        let mut pairs = [
            "auth_date=1700000000".to_string(),
            format!("query_id=test-query-{user_id}"),
            format!("user={user}"),
        ];
        pairs.sort();

        let secret = Sha256::digest("bot-token".as_bytes());
        let mut mac = HmacSha256::new_from_slice(secret.as_slice()).expect("hmac key");
        mac.update(pairs.join("\n").as_bytes());
        let hash = hex::encode(mac.finalize().into_bytes());

        format!(
            "query_id=test-query-{user_id}&user={}&auth_date=1700000000&hash={hash}",
            url::form_urlencoded::byte_serialize(user.as_bytes()).collect::<String>()
        )
    }

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
            db: AnyPoolOptions::new()
                .connect_lazy("postgres://postgres:postgres@127.0.0.1/postgres")
                .expect("lazy pool"),
            bucket: test_bucket(),
            admin_secret: "admin-secret".into(),
            config: crate::config::Config::parse_from([""]),
            jwt_secret: "jwt-secret".into(),
            profile_request_tx: tokio::sync::broadcast::channel(16).0,
            #[cfg(feature = "telegram-auth")]
            telegram_bot_token: "bot-token".into(),
            #[cfg(feature = "telegram-auth")]
            public_webapp_url: "https://example.com".into(),
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
        #[cfg(feature = "telegram-auth")]
        headers.insert(
            header::HeaderName::from_static("x-telegram-init-data"),
            HeaderValue::from_str(&telegram_init_data(12345)).expect("telegram header"),
        );

        let auth = User::from_headers(&state, &headers);
        assert!(auth.is_ok());
        let auth = auth.expect("authenticated user");
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

        let auth = User::from_headers(&state, &headers);
        assert!(auth.is_ok());
        let auth = auth.expect("authenticated user");
        assert!(auth.is_admin);
        assert_eq!(auth.user_id, Uuid::nil());
    }
}
