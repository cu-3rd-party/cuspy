use crate::ApiContext;
use crate::models::user::User;
use tonic::Request;
use tonic::Status;
use tonic::service::Interceptor;

#[derive(Clone)]
pub struct AuthInterceptor {
    state: ApiContext,
}

impl AuthInterceptor {
    pub fn new(state: ApiContext) -> Self {
        Self { state }
    }
}

impl Interceptor for AuthInterceptor {
    fn call(&mut self, mut req: Request<()>) -> Result<Request<()>, Status> {
        let user = User::from_metadata(&self.state, req.metadata()).ok();
        req.extensions_mut().insert(user);
        Ok(req)
    }
}
