use crate::grpc::RequestAuthExt;
use helloworld::greeter_server::Greeter;
use helloworld::{HelloReply, HelloRequest};
use tonic::{Request, Response, Status};

pub mod helloworld {
    tonic::include_proto!("helloworld");
}

#[tonic::async_trait]
impl Greeter for GreeterService {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        let _user = request.auth_user_cloned();
        let name = request.into_inner().name;
        let message = if name.is_empty() {
            "Hello!".to_string()
        } else {
            format!("Hello, {name}!")
        };

        Ok(Response::new(HelloReply { message }))
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct GreeterService;
