use crate::grpc::RequestAuthExt;
use crate::models::ApiError;
use crate::models::user::User;
use tonic::{Request, Status};

pub fn api_error_to_status(error: ApiError) -> Status {
    match error {
        ApiError::NotFound => Status::not_found("resource not found"),
        ApiError::Unauthorized | ApiError::Forbidden => Status::permission_denied("forbidden"),
        ApiError::BadRequest(message) => Status::invalid_argument(message),
        ApiError::Database(error) => Status::internal(error.to_string()),
        ApiError::Internal(message) => Status::internal(message),
        ApiError::PasswordHash => Status::internal("password hash error"),
        ApiError::Token => Status::internal("token error"),
    }
}

pub fn internal_error(error: impl std::fmt::Display) -> Status {
    Status::internal(error.to_string())
}

pub fn require_authenticated_user<T>(request: &Request<T>) -> Result<User, Status> {
    request
        .auth_user_cloned()
        .ok_or_else(|| Status::unauthenticated(""))
}

pub fn require_admin_user<T>(request: &Request<T>) -> Result<User, Status> {
    let user = require_authenticated_user(request)?;
    if user.is_admin {
        Ok(user)
    } else {
        Err(Status::permission_denied("admin access required"))
    }
}
