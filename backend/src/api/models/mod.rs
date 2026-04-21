use axum::response::{IntoResponse, Response};
use http::StatusCode;
use axum::Json;
use serde_json::json;
use serde::Serialize;

pub mod profile;
pub mod user;
pub mod auth;
pub mod similarity;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("resource not found")]
    NotFound,
    #[error("unauthorized")]
    Unauthorized,
    #[error("forbidden")]
    Forbidden,
    #[error("bad request: {0}")]
    BadRequest(String),
    #[error("database error")]
    Database(#[from] sqlx::Error),
    #[error("password hash error")]
    PasswordHash,
    #[error("token error")]
    Token,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = match self {
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::Forbidden => StatusCode::FORBIDDEN,
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::Database(_) | Self::PasswordHash | Self::Token => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = Json(json!({ "error": self.to_string() }));
        (status, body).into_response()
    }
}

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
}