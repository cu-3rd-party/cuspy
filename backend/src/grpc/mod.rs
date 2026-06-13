pub mod services;

use axum::Router;
use axum_tonic::NestTonic;
use services::greeter::helloworld::greeter_server::{Greeter, GreeterServer};
#[derive(Debug, Default, Clone, Copy)]
struct GreeterService;

pub fn router() -> Router {
    Router::new().nest_tonic(GreeterServer::new(GreeterService))
}
