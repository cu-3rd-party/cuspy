use crate::AppState;
use axum::{
    Router,
    routing::{get, post},
};

mod admin;
mod auth;
mod r#const;
mod health;
pub mod helpers;
mod image;
mod kills;
pub mod models;
#[path = "profile-creation.rs"]
mod profile_creation;
mod user;
mod db;
mod agent_data;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/health", get(health::health))
        .route("/auth/register", post(auth::register))
        .route("/auth/login", post(auth::login))
        .route("/auth/me", get(user::me))
        .route("/user/me", get(user::me).put(user::update_me))
        .route(
            "/users/{user_id}",
            get(user::get_user)
                .patch(user::update_user)
                .delete(user::delete_user),
        )
        .route(
            "/users/{left_user_id}/compare/{right_user_id}",
            get(user::compare_user_profiles),
        )
        .route("/system/profile-similarity", post(user::compare_profiles))
        .route(
            "/kills",
            get(kills::list_approved_kills).post(kills::report_kill),
        )
        .route("/kills/my-pending", get(kills::list_my_pending_kills))
        .route("/kills/{kill_id}/confirm", post(kills::confirm_kill))
        .route("/kills/{kill_id}/moderate", post(kills::moderate_kill))
        .route("/rankings", get(kills::rankings))
        .route("/stats/user/{user_id}", get(kills::user_stats))
        .route(
            "/profile-creation-requests",
            get(profile_creation::list_profile_creation_requests)
                .post(profile_creation::create_profile_creation_request),
        )
        .route(
            "/profile-creation-requests/{request_id}",
            get(profile_creation::get_profile_creation_request)
                .patch(profile_creation::update_profile_creation_request)
                .delete(profile_creation::delete_profile_creation_request),
        )
        .route(
            "/admin/users",
            get(admin::admin_list_users).post(admin::admin_create_user),
        )
        .route(
            "/admin/users/{user_id}",
            get(admin::admin_get_user)
                .patch(admin::admin_update_user)
                .delete(admin::admin_delete_user),
        )
        .route(
            "/admin/profile-creation-requests",
            get(admin::admin_list_profile_creation_requests),
        )
        .route(
            "/admin/profile-creation-requests/{request_id}",
            get(admin::admin_get_profile_creation_request)
                .patch(admin::admin_update_profile_creation_request)
                .delete(admin::admin_delete_profile_creation_request),
        )
}
