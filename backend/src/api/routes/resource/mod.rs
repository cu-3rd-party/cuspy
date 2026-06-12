pub mod get;

use axum::Router;
use axum::routing::get;
use crate::ApiContext;

pub fn router() -> Router<ApiContext> {
    Router::new()
        .route("{resource_id}", get(get::get_resource))
}
