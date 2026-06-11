use crate::ApiContext;
use crate::api::routes::profile_request::create::create_profile_request;
use crate::api::routes::profile_request::delete::delete_profile_request;
use crate::api::routes::profile_request::get::get_profile_request;
use crate::api::routes::profile_request::list::list_profile_requests;
use crate::api::routes::profile_request::update::update_profile_request;
use axum::Router;
use axum::routing::get;

mod create;
mod delete;
mod get;
mod list;
mod update;

pub fn router() -> Router<ApiContext> {
    Router::new()
        .route(
            "",
            get(list_profile_requests)
                .post(create_profile_request)
                .put(update_profile_request)
                .delete(delete_profile_request),
        )
        .route("/{request_id}", get(get_profile_request))
}
