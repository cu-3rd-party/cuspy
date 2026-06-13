use crate::ApiContext;
use axum::Router;
use axum::routing::get;

pub mod stats;

pub fn router() -> Router<ApiContext> {
    Router::new()
        .route("/rankings", get(stats::rankings))
        .route("/user/{user_id}", get(stats::user_stats))
}
