use tonic::{Request, Response, Status};
use helloworld::greeter_server::Greeter;
use helloworld::{HelloReply, HelloRequest};
use crate::grpc::GreeterService;

pub mod helloworld {
    tonic::include_proto!("helloworld");
}

#[tonic::async_trait]
impl Greeter for GreeterService {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        let name = request.into_inner().name;
        let message = if name.is_empty() {
            "Hello!".to_string()
        } else {
            format!("Hello, {name}!")
        };

        Ok(Response::new(HelloReply { message }))
    }
}