#![cfg(feature = "telegram")]

mod common;

use axum::http::StatusCode;
use serde_json::Value;

use common::{TestContext, seed_authenticated_user, telegram_init_data};

#[tokio::test]
async fn telegram_feature_keeps_bearer_auth_working() {
    let ctx = TestContext::new().await;
    let valid_init_data = telegram_init_data(7001);
    let invalid_init_data = telegram_init_data(9009);

    let (token, user) = seed_authenticated_user(&ctx, "TelegramUser").await;
    let user_id = user["user_id"].as_str().expect("user id").to_string();

    let (missing_header_status, missing_header_body) = ctx
        .json("GET", "/api/auth/me", None, Some(&token), None, None)
        .await;
    assert_eq!(missing_header_status, StatusCode::OK);
    assert_eq!(missing_header_body["user_id"], Value::String(user_id.clone()));

    let (me_status, me_body) = ctx
        .json(
            "GET",
            "/api/auth/me",
            None,
            Some(&token),
            None,
            Some(&valid_init_data),
        )
        .await;
    assert_eq!(me_status, StatusCode::OK);
    assert_eq!(me_body["user_id"], Value::String(user_id.clone()));

    let (invalid_header_status, invalid_header_body) = ctx
        .json(
            "GET",
            "/api/auth/me",
            None,
            Some(&token),
            None,
            Some(&invalid_init_data),
        )
        .await;
    assert_eq!(invalid_header_status, StatusCode::OK);
    assert_eq!(invalid_header_body["user_id"], Value::String(user_id));
}
