use axum::Router;
use axum::routing::{get};
use crate::api::{health};
use crate::ApiContext;

pub mod profile_request;
pub mod admin;
pub mod agent_data;
pub mod auth;
pub mod image;
pub mod kill;
pub mod stats;
pub mod user;

pub fn router() -> Router<ApiContext> {
    Router::new()
        .route("/health", get(health::health))
        .nest("/auth", auth::router())
        .nest("/user", user::router())
        .nest("/kill", kill::router())
        .nest("/stats", stats::router())
        .nest("/profile-requests", profile_request::router())
        .nest("/admin", admin::router())
}
