#![cfg(not(feature = "telegram-auth"))]

mod common;

use axum::http::StatusCode;
use serde_json::{Value, json};

use common::{TestContext, fetch_user_agent_data, register_user};

#[tokio::test]
async fn backend_endpoints_work_end_to_end() {
    let ctx = TestContext::new().await;

    let (health_status, health_body) = ctx.json("GET", "/health", None, None, None, None).await;
    assert_eq!(health_status, StatusCode::OK);
    assert_eq!(health_body["status"], "ok");

    let (root_status, root_body) = ctx.json("GET", "/", None, None, None, None).await;
    assert_eq!(root_status, StatusCode::OK);
    assert_eq!(root_body, Value::String("backend up".into()));

    let (token, user) = register_user(&ctx, "agent@example.com", 1001, "Alpha", None).await;
    let user_id = user["user_id"].as_str().expect("user id").to_string();

    let (login_status, login_body) = ctx
        .json(
            "POST",
            "/auth/login",
            Some(json!({ "email": "agent@example.com", "password": "password123" })),
            None,
            None,
            None,
        )
        .await;
    assert_eq!(login_status, StatusCode::OK);
    assert!(login_body["access_token"].as_str().is_some());

    let (me_status, me_body) = ctx.json("GET", "/auth/me", None, Some(&token), None, None).await;
    assert_eq!(me_status, StatusCode::OK);
    assert_eq!(me_body["user_id"], user["user_id"]);

    let (get_user_status, get_user_body) = ctx
        .json("GET", &format!("/users/{user_id}"), None, Some(&token), None, None)
        .await;
    assert_eq!(get_user_status, StatusCode::OK);
    assert_eq!(get_user_body["agent_name"], "Alpha");

    let (update_user_status, update_user_body) = ctx
        .json(
            "PATCH",
            &format!("/users/{user_id}"),
            Some(json!({
                "rating": 99,
                "agent_name": "Alpha Prime",
                "agent_data": { "track": "backend", "city": "Lviv", "course": 3 }
            })),
            Some(&token),
            None,
            None,
        )
        .await;
    assert_eq!(update_user_status, StatusCode::OK);
    assert_eq!(update_user_body["rating"], 99);

    let (compare_status, compare_body) = ctx
        .json(
            "GET",
            &format!("/users/{user_id}/compare/{user_id}"),
            None,
            Some(&token),
            None,
            None,
        )
        .await;
    assert_eq!(compare_status, StatusCode::OK);
    assert_eq!(compare_body["similarity_score"], 1.0);

    let (system_compare_status, system_compare_body) = ctx
        .json(
            "POST",
            "/system/profile-similarity",
            Some(json!({
                "left": { "a": 1, "b": 2 },
                "right": { "a": 1, "c": 3 }
            })),
            Some(&token),
            None,
            None,
        )
        .await;
    assert_eq!(system_compare_status, StatusCode::OK);
    assert_eq!(system_compare_body["matching_keys"], json!(["a"]));

    let (create_request_status, create_request_body) = ctx
        .json(
            "POST",
            "/profile-creation-requests",
            Some(json!({
                "requested_profile_data": { "track": "backend", "city": "Dnipro", "course": 4 }
            })),
            Some(&token),
            None,
            None,
        )
        .await;
    assert_eq!(create_request_status, StatusCode::CREATED);
    let request_id = create_request_body["profile_creation_request_id"]
        .as_str()
        .expect("request id")
        .to_string();

    let (list_requests_status, list_requests_body) = ctx
        .json("GET", "/profile-creation-requests", None, Some(&token), None, None)
        .await;
    assert_eq!(list_requests_status, StatusCode::OK);
    assert_eq!(list_requests_body.as_array().expect("array").len(), 1);

    let (get_request_status, get_request_body) = ctx
        .json(
            "GET",
            &format!("/profile-creation-requests/{request_id}"),
            None,
            Some(&token),
            None,
            None,
        )
        .await;
    assert_eq!(get_request_status, StatusCode::OK);
    assert_eq!(get_request_body["status"], "sent");

    let (update_request_status, update_request_body) = ctx
        .json(
            "PATCH",
            &format!("/profile-creation-requests/{request_id}"),
            Some(json!({
                "requested_profile_data": { "track": "backend", "city": "Kharkiv", "course": 5 }
            })),
            Some(&token),
            None,
            None,
        )
        .await;
    assert_eq!(update_request_status, StatusCode::OK);
    assert_eq!(update_request_body["requested_profile_data"]["city"], "Kharkiv");

    let (admin_list_users_status, admin_list_users_body) = ctx
        .json("GET", "/admin/users", None, None, Some(&ctx.admin_secret), None)
        .await;
    assert_eq!(admin_list_users_status, StatusCode::OK);
    assert_eq!(admin_list_users_body.as_array().expect("array").len(), 1);

    let (admin_create_user_status, admin_create_user_body) = ctx
        .json(
            "POST",
            "/admin/users",
            Some(json!({
                "telegram_id": 2002,
                "rating": 10,
                "agent_name": "Bravo",
                "agent_data": { "track": "frontend" }
            })),
            None,
            Some(&ctx.admin_secret),
            None,
        )
        .await;
    assert_eq!(admin_create_user_status, StatusCode::CREATED);
    let admin_created_user_id = admin_create_user_body["user_id"].as_str().expect("admin user id").to_string();

    let (admin_get_user_status, _) = ctx
        .json(
            "GET",
            &format!("/admin/users/{admin_created_user_id}"),
            None,
            None,
            Some(&ctx.admin_secret),
            None,
        )
        .await;
    assert_eq!(admin_get_user_status, StatusCode::OK);

    let (admin_update_user_status, admin_update_user_body) = ctx
        .json(
            "PATCH",
            &format!("/admin/users/{admin_created_user_id}"),
            Some(json!({ "rating": 77 })),
            None,
            Some(&ctx.admin_secret),
            None,
        )
        .await;
    assert_eq!(admin_update_user_status, StatusCode::OK);
    assert_eq!(admin_update_user_body["rating"], 77);

    let (admin_list_requests_status, admin_list_requests_body) = ctx
        .json(
            "GET",
            "/admin/profile-creation-requests",
            None,
            None,
            Some(&ctx.admin_secret),
            None,
        )
        .await;
    assert_eq!(admin_list_requests_status, StatusCode::OK);
    assert_eq!(admin_list_requests_body.as_array().expect("array").len(), 1);

    let (admin_get_request_status, _) = ctx
        .json(
            "GET",
            &format!("/admin/profile-creation-requests/{request_id}"),
            None,
            None,
            Some(&ctx.admin_secret),
            None,
        )
        .await;
    assert_eq!(admin_get_request_status, StatusCode::OK);

    let (admin_update_request_status, admin_update_request_body) = ctx
        .json(
            "PATCH",
            &format!("/admin/profile-creation-requests/{request_id}"),
            Some(json!({
                "status": "confirmed",
                "reviewer_note": "ok"
            })),
            None,
            Some(&ctx.admin_secret),
            None,
        )
        .await;
    assert_eq!(admin_update_request_status, StatusCode::OK);
    assert_eq!(admin_update_request_body["status"], "confirmed");

    let updated_agent_data = fetch_user_agent_data(&ctx, &user_id).await;
    assert_eq!(updated_agent_data["city"], "Kharkiv");

    let (other_token, other_user) = register_user(&ctx, "other@example.com", 3003, "Other", None).await;
    let other_user_id = other_user["user_id"].as_str().expect("other user id");

    let (forbidden_user_status, _) = ctx
        .json(
            "GET",
            &format!("/users/{user_id}"),
            None,
            Some(&other_token),
            None,
            None,
        )
        .await;
    assert_eq!(forbidden_user_status, StatusCode::FORBIDDEN);

    let (forbidden_request_status, _) = ctx
        .json(
            "GET",
            &format!("/profile-creation-requests/{request_id}"),
            None,
            Some(&other_token),
            None,
            None,
        )
        .await;
    assert_eq!(forbidden_request_status, StatusCode::FORBIDDEN);

    let (admin_delete_request_status, _) = ctx
        .json(
            "DELETE",
            &format!("/admin/profile-creation-requests/{request_id}"),
            None,
            None,
            Some(&ctx.admin_secret),
            None,
        )
        .await;
    assert_eq!(admin_delete_request_status, StatusCode::NO_CONTENT);

    let (admin_delete_other_status, _) = ctx
        .json(
            "DELETE",
            &format!("/admin/users/{admin_created_user_id}"),
            None,
            None,
            Some(&ctx.admin_secret),
            None,
        )
        .await;
    assert_eq!(admin_delete_other_status, StatusCode::NO_CONTENT);

    let (delete_user_status, _) = ctx
        .json("DELETE", &format!("/users/{other_user_id}"), None, Some(&other_token), None, None)
        .await;
    assert_eq!(delete_user_status, StatusCode::NO_CONTENT);
}
