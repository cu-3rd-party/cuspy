pub mod get;

use crate::ApiContext;
use axum::Router;
use axum::routing::get;

pub fn router() -> Router<ApiContext> {
    Router::new().route("/{resource_id}", get(get::get_resource))
}
