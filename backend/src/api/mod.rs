use crate::ApiContext;
use crate::api::routes::stats;
use axum::{
    Router,
    routing::{get, post},
};
use routes::admin::{profile_request_crud, users_crud};
use routes::auth::{login, register};
use routes::kill;

mod apiuser;
mod r#const;
mod db;
mod health;
pub mod helpers;
pub mod models;
#[path = "apiprofile_creation.rs"]
mod profile_creation;
pub mod routes;

pub fn router() -> Router<ApiContext> {
    Router::new()
        .route("/health", get(health::health))
        .route("/auth/register", post(register::register))
        .route("/auth/login", post(login::login))
        .route("/auth/me", get(apiuser::me))
        .route("/user/me", get(apiuser::me).put(apiuser::update_me))
        .route(
            "/users/{user_id}",
            get(apiuser::get_user)
                .patch(apiuser::update_user)
                .delete(apiuser::delete_user),
        )
        .route(
            "/users/{left_user_id}/compare/{right_user_id}",
            get(apiuser::compare_user_profiles),
        )
        .route(
            "/system/profile-similarity",
            post(apiuser::compare_profiles),
        )
        .nest("/kill", stats::router())
        .nest("/stats", stats::router())
        .route(
            "/profile-creation-requests",
            get(profile_creation::list_profile_requests)
                .post(profile_creation::create_profile_request),
        )
        .route(
            "/profile-creation-requests/{request_id}",
            get(profile_creation::get_profile_request)
                .patch(profile_creation::update_profile_request)
                .delete(profile_creation::delete_profile_request),
        )
        .route(
            "/admin/users",
            get(users_crud::admin_list_users).post(users_crud::admin_create_user),
        )
        .route(
            "/admin/users/{user_id}",
            get(users_crud::admin_get_user)
                .patch(users_crud::admin_update_user)
                .delete(users_crud::admin_delete_user),
        )
        .route(
            "/admin/profile-creation-requests",
            get(profile_request_crud::admin_list_profile_requests),
        )
        .route(
            "/admin/profile-creation-requests/{request_id}",
            get(profile_request_crud::admin_get_profile_request)
                .patch(profile_request_crud::admin_update_profile_request)
                .delete(profile_request_crud::admin_delete_profile_request),
        )
}
