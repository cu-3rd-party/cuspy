use axum::{
    routing::{get, post}
    ,
    Router,
};
use crate::AppState;

mod r#const;
mod models;
mod health;
mod helpers;
mod auth;
mod user;
#[path = "profile-creation.rs"]
mod profile_creation;
mod admin;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/health", get(health::health))
        .route("/auth/register", post(auth::register))
        .route("/auth/login", post(auth::login))
        .route("/auth/me", get(user::me))
        .route("/users/{user_id}", get(user::get_user).patch(user::update_user).delete(user::delete_user))
        .route("/users/{left_user_id}/compare/{right_user_id}", get(user::compare_user_profiles))
        .route("/system/profile-similarity", post(user::compare_profiles))
        .route(
            "/profile-creation-requests",
            get(profile_creation::list_profile_creation_requests).post(profile_creation::create_profile_creation_request),
        )
        .route(
            "/profile-creation-requests/{request_id}",
            get(profile_creation::get_profile_creation_request)
                .patch(profile_creation::update_profile_creation_request)
                .delete(profile_creation::delete_profile_creation_request),
        )
        .route("/admin/users", get(admin::admin_list_users).post(admin::admin_create_user))
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
