use crate::ApiContext;
use axum::Router;
use axum::routing::post;

pub mod create;

pub fn router() -> Router<ApiContext> {
    Router::new().route("/", post(create::create_agent_data))
}
