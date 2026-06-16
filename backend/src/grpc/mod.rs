mod middleware;
mod services;

use crate::ApiContext;
use crate::grpc::middleware::auth_interceptor::AuthInterceptor;
use crate::grpc::services::greeter::GreeterService;
use crate::grpc::services::greeter::helloworld::greeter_server::GreeterServer;
use crate::grpc::services::profile_request::ProfileRequestService;
use crate::grpc::services::profile_request::profilerequest::profile_request_server::ProfileRequestServer;
use axum::Router;
use axum_tonic::NestTonic;
use tonic::Request;
use tonic::service::InterceptorLayer;
use tonic_reflection::server::Builder as ReflectionBuilder;
use tower::Layer;

use crate::models::user::User;

pub trait RequestAuthExt {
    fn auth_user(&self) -> Option<&User>;

    fn auth_user_cloned(&self) -> Option<User>
    where
        User: Clone,
    {
        self.auth_user().cloned()
    }
}

impl<T> RequestAuthExt for Request<T> {
    fn auth_user(&self) -> Option<&User> {
        self.extensions().get::<Option<User>>()?.as_ref()
    }
}

#[allow(unused)]
pub fn router(state: ApiContext) -> Router {
    let reflection_service = ReflectionBuilder::configure()
        .register_encoded_file_descriptor_set(include_bytes!(concat!(
            env!("OUT_DIR"),
            "/helloworld_descriptor.bin"
        )))
        .register_encoded_file_descriptor_set(include_bytes!(concat!(
            env!("OUT_DIR"),
            "/profilerequest_descriptor.bin"
        )))
        .build_v1()
        .expect("grpc reflection service");

    let profile_tx = state.profile_request_tx.clone();
    let auth_layer = InterceptorLayer::new(AuthInterceptor::new(state.clone()));
    let grpc_service = auth_layer.layer(GreeterServer::new(GreeterService));
    let profile_request_service = auth_layer.layer(ProfileRequestServer::new(
        ProfileRequestService::new(state.db.clone(), profile_tx),
    ));

    Router::new()
        .nest_tonic(reflection_service)
        .nest_tonic(grpc_service)
        .nest_tonic(profile_request_service)
}
