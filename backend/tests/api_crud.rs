mod common;

use axum::http::StatusCode;
use serde_json::Value;

use common::{
    TestContext, create_profile_request, fetch_latest_audit_actor, seed_authenticated_user,
};

#[tokio::test]
async fn backend_endpoints_work_end_to_end() {
    let ctx = TestContext::new().await;

    let (health_status, health_body) = ctx.json("GET", "/api/health", None, None, None, None).await;
    assert_eq!(health_status, StatusCode::OK);
    assert_eq!(health_body["status"], "ok");

    let (root_status, root_body) = ctx.json("GET", "/api", None, None, None, None).await;
    assert_eq!(root_status, StatusCode::OK);
    assert_eq!(root_body, Value::String("backend up".into()));

    let (token, user) = seed_authenticated_user(&ctx, "Alpha").await;
    let user_id = user["user_id"].as_str().expect("user id").to_string();

    let (me_status, me_body) = ctx
        .json("GET", "/api/auth/me", None, Some(&token), None, None)
        .await;
    assert_eq!(me_status, StatusCode::OK);
    assert_eq!(me_body["user_id"], user["user_id"]);
    assert_eq!(fetch_latest_audit_actor(&ctx, "/api/auth/me").await, Some(user_id.clone()));

    let (get_user_status, get_user_body) = ctx
        .json(
            "GET",
            &format!("/api/user/{user_id}"),
            None,
            Some(&token),
            None,
            None,
        )
        .await;
    assert_eq!(get_user_status, StatusCode::OK);
    assert_eq!(get_user_body["username"], "Alpha");
    assert_eq!(get_user_body["rating"], 0);

    let (update_user_status, update_user_body) = ctx
        .json(
            "PATCH",
            &format!("/api/user/{user_id}"),
            Some(serde_json::json!({ "username": "Alpha Prime" })),
            Some(&token),
            None,
            None,
        )
        .await;
    assert_eq!(update_user_status, StatusCode::OK);
    assert_eq!(update_user_body["username"], "Alpha Prime");

    let create_request_body = create_profile_request(&ctx, "Delta", &token, None).await;
    let request_id = create_request_body["profile_request_id"]
        .as_str()
        .expect("request id")
        .to_string();
    assert_eq!(create_request_body["status"], "sent");
    assert_eq!(
        create_request_body["requested_profile_data"]["codename"],
        Value::String("Delta".to_string())
    );

    let (list_requests_status, list_requests_body) = ctx
        .json(
            "GET",
            "/api/profile-requests?all=false",
            None,
            Some(&token),
            None,
            None,
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
            None,
        )
        .await;
    assert_eq!(get_request_status, StatusCode::OK);
    assert_eq!(get_request_body["status"], "sent");

    let (admin_list_requests_status, admin_list_requests_body) = ctx
        .json(
            "GET",
            "/api/profile-requests?all=true",
            None,
            None,
            Some(&ctx.admin_secret),
            None,
        )
        .await;
    assert_eq!(admin_list_requests_status, StatusCode::OK);
    assert_eq!(admin_list_requests_body.as_array().expect("array").len(), 1);

    let (update_request_status, update_request_body) = ctx
        .json(
            "PUT",
            &format!("/api/profile-requests/{request_id}"),
            Some(serde_json::json!({
                "status": "confirmed",
                "reviewer_note": "ok"
            })),
            None,
            Some(&ctx.admin_secret),
            None,
        )
        .await;
    assert_eq!(update_request_status, StatusCode::OK);
    assert_eq!(update_request_body["status"], "confirmed");
    assert_eq!(update_request_body["reviewer_note"], "ok");
    assert!(update_request_body["reviewed_at"].is_string());

    let (rankings_status, rankings_body) = ctx
        .json(
            "GET",
            "/api/stats/rankings",
            None,
            Some(&token),
            None,
            None,
        )
        .await;
    assert_eq!(rankings_status, StatusCode::OK);
    let rankings = rankings_body.as_array().expect("rankings array");
    let user_ranking = rankings
        .iter()
        .find(|entry| entry["user_id"] == user_id)
        .expect("user ranking");
    assert_eq!(user_ranking["rating"], 1000);
    assert_eq!(user_ranking["approved_kills"], 0);

    let (stats_status, stats_body) = ctx
        .json(
            "GET",
            &format!("/api/stats/user/{user_id}"),
            None,
            Some(&token),
            None,
            None,
        )
        .await;
    assert_eq!(stats_status, StatusCode::OK);
    assert_eq!(stats_body["approved_kills"], 0);
    assert_eq!(stats_body["rating"], 1000);

    let (other_token, other_user) = seed_authenticated_user(&ctx, "Other").await;
    let other_user_id = other_user["user_id"].as_str().expect("other user id").to_string();

    let (public_user_status, public_user_body) = ctx
        .json(
            "GET",
            &format!("/api/user/{user_id}"),
            None,
            Some(&other_token),
            None,
            None,
        )
        .await;
    assert_eq!(public_user_status, StatusCode::OK);
    assert_eq!(public_user_body["user_id"], user["user_id"]);

    let (forbidden_request_status, _) = ctx
        .json(
            "GET",
            &format!("/api/profile-requests/{request_id}"),
            None,
            Some(&other_token),
            None,
            None,
        )
        .await;
    assert_eq!(forbidden_request_status, StatusCode::FORBIDDEN);

    let (delete_request_status, _) = ctx
        .json(
            "DELETE",
            &format!("/api/profile-requests/{request_id}"),
            None,
            None,
            Some(&ctx.admin_secret),
            None,
        )
        .await;
    assert_eq!(delete_request_status, StatusCode::NO_CONTENT);

    let (delete_other_status, _) = ctx
        .json(
            "DELETE",
            &format!("/api/user/{other_user_id}"),
            None,
            Some(&other_token),
            None,
            None,
        )
        .await;
    assert_eq!(delete_other_status, StatusCode::NO_CONTENT);

    let (delete_user_status, _) = ctx
        .json(
            "DELETE",
            &format!("/api/user/{user_id}"),
            None,
            Some(&token),
            None,
            None,
        )
        .await;
    assert_eq!(delete_user_status, StatusCode::NO_CONTENT);
}
