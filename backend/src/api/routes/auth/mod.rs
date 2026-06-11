use crate::ApiContext;
use axum::Router;
use axum::routing::{get, post};

pub mod login;
pub mod me;
pub mod register;

pub fn router() -> Router<ApiContext> {
    Router::new()
        .route("/register", post(register::register))
        .route("/login", post(login::login))
        .route("/me", get(me::me))
}
