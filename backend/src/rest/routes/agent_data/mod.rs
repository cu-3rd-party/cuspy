use crate::ApiContext;
use axum::Router;
use axum::routing::{get, post};

pub mod create;
pub mod get;

pub fn router() -> Router<ApiContext> {
    Router::new()
        .route("/", post(create::create_agent_data))
        .route("/{agent_data_id}", get(get::get_agent_data))
}
