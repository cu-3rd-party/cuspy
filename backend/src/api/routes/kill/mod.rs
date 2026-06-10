mod confirm;
mod helpers;
mod list;
mod moderate;
mod report;

use crate::ApiContext;
use axum::Router;
use axum::routing::{get, post};

pub fn router() -> Router<ApiContext> {
    Router::new()
        .route("", get(list::list_kills).post(report::report_kill))
        .route("/{kill_id}/confirm", post(confirm::confirm_kill))
        .route("/{kill_id}/moderate", post(moderate::moderate_kill))
}
