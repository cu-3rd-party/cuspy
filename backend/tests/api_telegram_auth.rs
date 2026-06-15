#![cfg(feature = "telegram-auth")]

mod common;

use axum::http::StatusCode;
use serde_json::{Value, json};

use common::{TestContext, create_agent_data, register_user, telegram_init_data};

#[tokio::test]
async fn telegram_auth_requires_valid_init_data() {
    let ctx = TestContext::new().await;
    let valid_init_data = telegram_init_data(7001);
    let invalid_init_data = telegram_init_data(9009);

    let (register_status, register_body) = ctx
        .json(
            "POST",
            "/api/auth/register",
            Some(json!({
                "email": "tg@example.com",
                "password": "password123",
                "telegram_id": 7001,
                "rating": 7,
                "username": "Telegram",
                "agent_data": { "track": "tg" }
            })),
            None,
            None,
            Some(&valid_init_data),
        )
        .await;
    assert_eq!(register_status, StatusCode::CREATED);
    let token = register_body["access_token"]
        .as_str()
        .expect("token")
        .to_string();
    let user_id = register_body["user"]["user_id"]
        .as_str()
        .expect("user id")
        .to_string();

    let (missing_header_status, _) = ctx
        .json("GET", "/api/auth/me", None, Some(&token), None, None)
        .await;
    assert_eq!(missing_header_status, StatusCode::UNAUTHORIZED);

    let (bad_header_status, bad_header_body) = ctx
        .json(
            "GET",
            "/api/auth/me",
            None,
            Some(&token),
            None,
            Some(&invalid_init_data),
        )
        .await;
    assert_eq!(bad_header_status, StatusCode::OK);
    assert_eq!(bad_header_body["user_id"], Value::String(user_id.clone()));

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

    let (login_status, login_body) = ctx
        .json(
            "POST",
            "/api/auth/login",
            Some(json!({
                "email": "tg@example.com",
                "password": "password123"
            })),
            None,
            None,
            Some(&valid_init_data),
        )
        .await;
    assert_eq!(login_status, StatusCode::OK);
    assert!(login_body["access_token"].as_str().is_some());

    let agent_data = create_agent_data(&ctx, "Telegram Profile").await;

    let (unauthorized_login_status, _) = ctx
        .json(
            "POST",
            "/api/auth/login",
            Some(json!({
                "email": "tg@example.com",
                "password": "password123"
            })),
            None,
            None,
            Some(&invalid_init_data),
        )
        .await;
    assert_eq!(unauthorized_login_status, StatusCode::UNAUTHORIZED);

    let (request_status, request_body) = ctx
        .json(
            "POST",
            "/api/profile-requests",
            Some(json!({ "agent_data_id": agent_data["agent_data_id"] })),
            Some(&token),
            None,
            Some(&valid_init_data),
        )
        .await;
    assert_eq!(request_status, StatusCode::CREATED);
    let request_id = request_body["profile_request_id"]
        .as_str()
        .expect("request id");

    let (forbidden_other_register_token, _) = register_user(
        &ctx,
        "tg-other@example.com",
        8002,
        "Other TG",
        Some(&telegram_init_data(8002)),
    )
    .await;

    let (forbidden_user_status, _) = ctx
        .json(
            "GET",
            &format!("/api/user/{user_id}"),
            None,
            Some(&forbidden_other_register_token),
            None,
            Some(&telegram_init_data(8002)),
        )
        .await;
    assert_eq!(forbidden_user_status, StatusCode::FORBIDDEN);

    let (forbidden_request_status, forbidden_request_body) = ctx
        .json(
            "GET",
            &format!("/api/profile-requests/{request_id}"),
            None,
            Some(&token),
            None,
            Some(&invalid_init_data),
        )
        .await;
    assert_eq!(forbidden_request_status, StatusCode::OK);
    assert_eq!(
        forbidden_request_body["profile_request_id"],
        Value::String(request_id.to_string())
    );
}
