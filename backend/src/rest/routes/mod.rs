use crate::ApiContext;
use crate::rest::health;
use axum::Router;
use axum::routing::get;

pub mod auth;
pub mod kill;
pub mod profile_request;
pub mod resource;
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
        .nest("/resource", resource::router())
}
