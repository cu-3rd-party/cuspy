pub mod docs;
pub mod extractor;
mod health;
pub mod helpers;
pub mod routes;

pub use routes::router;
use utoipa::OpenApi;

#[utoipa::path(
    get,
    path = "/",
    tag = "system",
    responses(
        (status = 200, description = "Backend is running", body = String)
    )
)]
pub async fn root() -> &'static str {
    "backend up"
}
