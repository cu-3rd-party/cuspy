use crate::api::routes::admin::profile_request_crud::profile_request_router;
use crate::api::routes::admin::users_crud::users_router;
use crate::ApiContext;
use axum::Router;

pub mod profile_request_crud;
pub mod users_crud;

pub fn router() -> Router<ApiContext> {
    Router::new()
        .nest("/profile-requests", profile_request_router())
        .nest("/user", users_router())
}
