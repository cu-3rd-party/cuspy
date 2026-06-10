use axum::Router;
use axum::routing::{get, post};
use crate::ApiContext;

pub mod login;
pub mod register;
pub mod me;

pub fn router() -> Router<ApiContext> {
    Router::new()
        .route("/register", post(register::register))
        .route("/login", post(login::login))
        .route("/me", get(me::me))
}
