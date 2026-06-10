use axum::Router;
use axum::routing::get;
use crate::ApiContext;

mod get;
mod update;
mod delete;

pub fn router() -> Router<ApiContext> {
    Router::new()
        .route("{user_id}",
               get(get::get_user)
                   .delete(delete::delete_user)
                   .patch(update::update_user))
}
