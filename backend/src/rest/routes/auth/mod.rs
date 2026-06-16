use crate::ApiContext;
use axum::Router;
use axum::routing::{get, post};

pub mod login;
pub mod me;
pub mod refresh;
pub mod register;
#[cfg(feature = "telegram")]
pub mod telegram;

pub fn router() -> Router<ApiContext> {
    let router = Router::new()
        .route("/register", post(register::register))
        .route("/login", post(login::login))
        .route("/me", get(me::me))
        .route("/refresh", post(refresh::refresh_token_pair));
    #[cfg(feature = "telegram")]
    let router = router.route("/telegram", post(telegram::telegram_login_request));
    router
}
