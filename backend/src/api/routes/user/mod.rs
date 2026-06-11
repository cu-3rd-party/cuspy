use crate::ApiContext;
use axum::Router;
use axum::routing::get;

pub mod delete;
pub mod get;
pub mod update;

pub fn router() -> Router<ApiContext> {
    Router::new().route(
        "/{user_id}",
        get(get::get_user)
            .delete(delete::delete_user)
            .patch(update::update_user),
    )
}
