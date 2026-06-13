mod common;

use axum::http::StatusCode;
use serde_json::{Value, json};

#[cfg(feature = "telegram-auth")]
use common::telegram_init_data;
use common::{
    TestContext, create_agent_data, fetch_latest_audit_actor, fetch_user_agent_data, register_user,
    seed_admin_user, seed_resource,
};

#[tokio::test]
async fn backend_endpoints_work_end_to_end() {
    let ctx = TestContext::new().await;

    let (health_status, health_body) = ctx
        .json("GET", "/api/health", None, None, None, None)
        .await;
    assert_eq!(health_status, StatusCode::OK);
    assert_eq!(health_body["status"], "ok");

    let (root_status, root_body) = ctx
        .json("GET", "/api", None, None, None, None)
        .await;
    assert_eq!(root_status, StatusCode::OK);
    assert_eq!(root_body, Value::String("backend up".into()));

    let agent_data = create_agent_data(&ctx, "Delta").await;
    let agent_data_id = agent_data["agent_data_id"]
        .as_str()
        .expect("agent data id")
        .to_string();

    let resource_id = seed_resource(&ctx).await;
    let (resource_status, resource_body) = ctx
        .json(
            "GET",
            &format!("/api/resource/{resource_id}"),
            None,
            None,
            None,
            None,
        )
        .await;
    assert_eq!(resource_status, StatusCode::OK);
    let location =
        resource_body["file_location"]
            .to_string();
    assert!(location.contains("http://127.0.0.1:9000"));
    assert!(location.contains("test-bucket/"));
    assert!(location.contains("test/resource.txt"));

    #[cfg(feature = "telegram-auth")]
    let user_init_data = telegram_init_data(1001);
    #[cfg(feature = "telegram-auth")]
    let user_auth = Some(user_init_data.as_str());
    #[cfg(not(feature = "telegram-auth"))]
    let user_auth: Option<&str> = None;

    let (token, user) = register_user(&ctx, "agent@example.com", 1001, "Alpha", user_auth).await;
    let user_id = user["user_id"].as_str().expect("user id").to_string();

    let (login_status, login_body) = ctx
        .json(
            "POST",
            "/api/auth/login",
            Some(json!({ "email": "agent@example.com", "password": "password123" })),
            None,
            None,
            user_auth,
        )
        .await;
    assert_eq!(login_status, StatusCode::OK);
    assert!(login_body["access_token"].as_str().is_some());

    let (me_status, me_body) = ctx
        .json("GET", "/api/auth/me", None, Some(&token), None, user_auth)
        .await;
    assert_eq!(me_status, StatusCode::OK);
    assert_eq!(me_body["user_id"], user["user_id"]);
    assert_eq!(
        fetch_latest_audit_actor(&ctx, "/api/auth/me").await,
        Some(user_id.clone())
    );

    let (get_user_status, get_user_body) = ctx
        .json(
            "GET",
            &format!("/api/user/{user_id}"),
            None,
            Some(&token),
            None,
            user_auth,
        )
        .await;
    assert_eq!(get_user_status, StatusCode::OK);
    assert_eq!(get_user_body["agent_name"], "Alpha");
    assert_eq!(get_user_body["rating"], 1000);

    let (update_user_status, update_user_body) = ctx
        .json(
            "PATCH",
            &format!("/api/user/{user_id}"),
            Some(json!({
                "agent_name": "Alpha Prime"
            })),
            Some(&token),
            None,
            user_auth,
        )
        .await;
    assert_eq!(update_user_status, StatusCode::OK);
    assert_eq!(update_user_body["rating"], 1000);
    assert_eq!(update_user_body["agent_name"], "Alpha Prime");

    let (create_request_status, create_request_body) = ctx
        .json(
            "POST",
            "/api/profile-requests",
            Some(json!({ "agent_data_id": agent_data_id })),
            Some(&token),
            None,
            user_auth,
        )
        .await;
    assert_eq!(create_request_status, StatusCode::CREATED);
    let request_id = create_request_body["profile_request_id"]
        .as_str()
        .expect("request id")
        .to_string();

    let (list_requests_status, list_requests_body) = ctx
        .json(
            "GET",
            "/api/profile-requests",
            None,
            Some(&token),
            None,
            user_auth,
        )
        .await;
    assert_eq!(list_requests_status, StatusCode::OK);
    assert_eq!(list_requests_body.as_array().expect("array").len(), 1);

    let (get_request_status, get_request_body) = ctx
        .json(
            "GET",
            &format!("/api/profile-requests/{request_id}"),
            None,
            Some(&token),
            None,
            user_auth,
        )
        .await;
    assert_eq!(get_request_status, StatusCode::OK);
    assert_eq!(get_request_body["status"], "sent");

    let (update_request_status, update_request_body) = ctx
        .json(
            "PUT",
            &format!("/api/profile-requests/{request_id}"),
            Some(json!({
                "requested_profile_data": { "codename": "Kharkiv", "course_number": 5 }
            })),
            Some(&token),
            None,
            user_auth,
        )
        .await;
    assert_eq!(update_request_status, StatusCode::OK);
    assert_eq!(
        update_request_body["requested_profile_data_id"],
        create_request_body["requested_profile_data_id"]
    );

    let (admin_list_users_status, admin_list_users_body) = ctx
        .json(
            "GET",
            "/api/admin/user",
            None,
            None,
            Some(&ctx.admin_secret),
            None,
        )
        .await;
    assert_eq!(admin_list_users_status, StatusCode::OK);
    assert_eq!(admin_list_users_body.as_array().expect("array").len(), 1);

    let (admin_create_user_status, admin_create_user_body) = ctx
        .json(
            "POST",
            "/api/admin/user",
            Some(json!({
                "telegram_id": 2002,
                "agent_name": "Bravo",
                "agent_data": { "track": "frontend" },
                "is_admin": true
            })),
            None,
            Some(&ctx.admin_secret),
            None,
        )
        .await;
    assert_eq!(admin_create_user_status, StatusCode::CREATED);
    let admin_created_user_id = admin_create_user_body["user_id"]
        .as_str()
        .expect("admin user id")
        .to_string();

    let (admin_get_user_status, _) = ctx
        .json(
            "GET",
            &format!("/api/admin/user/{admin_created_user_id}"),
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
            &format!("/api/admin/user/{admin_created_user_id}"),
            Some(json!({ "agent_name": "Bravo Lead", "is_admin": false })),
            None,
            Some(&ctx.admin_secret),
            None,
        )
        .await;
    assert_eq!(admin_update_user_status, StatusCode::OK);
    assert_eq!(admin_update_user_body["rating"], 1000);
    assert_eq!(admin_update_user_body["is_admin"], false);

    let (admin_list_requests_status, admin_list_requests_body) = ctx
        .json(
            "GET",
            "/api/admin/profile-requests",
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
            &format!("/api/admin/profile-requests/{request_id}"),
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
            &format!("/api/admin/profile-requests/{request_id}"),
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
    assert_eq!(updated_agent_data["codename"], "Kharkiv");

    #[cfg(feature = "telegram-auth")]
    let other_init_data = telegram_init_data(3003);
    #[cfg(feature = "telegram-auth")]
    let other_auth = Some(other_init_data.as_str());
    #[cfg(not(feature = "telegram-auth"))]
    let other_auth: Option<&str> = None;

    let (other_token, other_user) =
        register_user(&ctx, "other@example.com", 3003, "Other", other_auth).await;
    let other_user_id = other_user["user_id"].as_str().expect("other user id");
    let admin_token = seed_admin_user(&ctx, "admin@example.com", 4004, "Control").await;

    #[cfg(feature = "telegram-auth")]
    let admin_init_data = telegram_init_data(4004);
    #[cfg(feature = "telegram-auth")]
    let admin_auth = Some(admin_init_data.as_str());
    #[cfg(not(feature = "telegram-auth"))]
    let admin_auth: Option<&str> = None;

    let (pending_kills_status, pending_kills_body) = ctx
        .json("GET", "/api/kill", None, Some(&other_token), None, other_auth)
        .await;
    assert_eq!(pending_kills_status, StatusCode::OK);
    assert_eq!(pending_kills_body.as_array().expect("array").len(), 0);

    let (report_kill_status, report_kill_body) = ctx
        .json(
            "POST",
            "/api/kill",
            Some(json!({
                "victim_id": other_user_id,
                "evidence_url": "https://example.com/evidence",
                "details": { "location": "library", "witnesses": 1 }
            })),
            Some(&token),
            None,
            user_auth,
        )
        .await;
    assert_eq!(report_kill_status, StatusCode::CREATED);
    assert_eq!(report_kill_body["status"], "REPORTED");
    let kill_id = report_kill_body["kill_event_id"]
        .as_str()
        .expect("kill id")
        .to_string();

    let (victim_pending_status, victim_pending_body) = ctx
        .json(
            "GET",
            &format!("/api/kill?victim_user_id={other_user_id}"),
            None,
            Some(&other_token),
            None,
            other_auth,
        )
        .await;
    assert_eq!(victim_pending_status, StatusCode::OK);
    assert_eq!(victim_pending_body.as_array().expect("array").len(), 1);

    let (confirm_kill_status, confirm_kill_body) = ctx
        .json(
            "POST",
            &format!("/api/kill/{kill_id}/confirm"),
            Some(json!({ "confirmed": true })),
            Some(&other_token),
            None,
            other_auth,
        )
        .await;
    assert_eq!(confirm_kill_status, StatusCode::OK);
    assert_eq!(confirm_kill_body["status"], "CONFIRMED");

    let (moderate_kill_status, moderate_kill_body) = ctx
        .json(
            "POST",
            &format!("/api/kill/{kill_id}/moderate"),
            Some(json!({ "action": "APPROVE", "reason": "verified" })),
            Some(&admin_token),
            None,
            admin_auth,
        )
        .await;
    assert_eq!(moderate_kill_status, StatusCode::OK);
    assert_eq!(moderate_kill_body["status"], "ADMIN_APPROVED");
    assert_eq!(
        fetch_latest_audit_actor(&ctx, "/api/kill/{kill_id}/moderate").await,
        Some(
            moderate_kill_body["moderator_id"]
                .as_str()
                .expect("moderator id")
                .to_string()
        )
    );

    let (approved_kills_status, approved_kills_body) = ctx
        .json("GET", "/api/kill", None, Some(&token), None, user_auth)
        .await;
    assert_eq!(approved_kills_status, StatusCode::OK);
    assert_eq!(approved_kills_body.as_array().expect("array").len(), 1);

    let (rankings_status, rankings_body) = ctx
        .json(
            "GET",
            "/api/stats/rankings",
            None,
            Some(&token),
            None,
            user_auth,
        )
        .await;
    assert_eq!(rankings_status, StatusCode::OK);
    let rankings = rankings_body.as_array().expect("rankings array");
    let user_ranking = rankings
        .iter()
        .find(|entry| entry["user_id"] == user_id)
        .expect("user ranking");
    assert_eq!(user_ranking["rating"], 1025);
    assert_eq!(user_ranking["approved_kills"], 1);

    let (stats_status, stats_body) = ctx
        .json(
            "GET",
            &format!("/api/stats/user/{user_id}"),
            None,
            Some(&token),
            None,
            user_auth,
        )
        .await;
    assert_eq!(stats_status, StatusCode::OK);
    assert_eq!(stats_body["approved_kills"], 1);
    assert_eq!(stats_body["rating"], 1025);

    let (forbidden_user_status, _) = ctx
        .json(
            "GET",
            &format!("/api/user/{user_id}"),
            None,
            Some(&other_token),
            None,
            other_auth,
        )
        .await;
    assert_eq!(forbidden_user_status, StatusCode::FORBIDDEN);

    let (forbidden_request_status, _) = ctx
        .json(
            "GET",
            &format!("/api/profile-requests/{request_id}"),
            None,
            Some(&other_token),
            None,
            other_auth,
        )
        .await;
    assert_eq!(forbidden_request_status, StatusCode::FORBIDDEN);

    let (admin_delete_request_status, _) = ctx
        .json(
            "DELETE",
            &format!("/api/admin/profile-requests/{request_id}"),
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
            &format!("/api/admin/user/{admin_created_user_id}"),
            None,
            None,
            Some(&ctx.admin_secret),
            None,
        )
        .await;
    assert_eq!(admin_delete_other_status, StatusCode::NO_CONTENT);

    let (delete_user_status, _) = ctx
        .json(
            "DELETE",
            &format!("/api/user/{other_user_id}"),
            None,
            Some(&other_token),
            None,
            other_auth,
        )
        .await;
    assert_eq!(delete_user_status, StatusCode::NO_CONTENT);
}
