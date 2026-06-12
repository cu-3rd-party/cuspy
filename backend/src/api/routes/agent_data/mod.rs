use axum::Router;
use axum::routing::post;
use crate::ApiContext;

pub mod create;

pub fn router() -> Router<ApiContext> {
    Router::new()
        .route("", post(create::create_agent_data))
}
