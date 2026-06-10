use axum::Json;
use axum::response::{IntoResponse, Response};
use http::StatusCode;
use serde::Serialize;
use serde_json::json;
use sqlx::{Row, any::AnyRow};
use uuid::Uuid;

fn decode_timestamp_value(value: &str) -> Result<sqlx::types::time::OffsetDateTime, sqlx::Error> {
    use time::{
        OffsetDateTime, PrimitiveDateTime, UtcOffset, format_description::well_known::Rfc3339,
    };

    if let Ok(unix) = value.parse::<i64>() {
        return sqlx::types::time::OffsetDateTime::from_unix_timestamp(unix)
            .map_err(|error| sqlx::Error::Decode(Box::new(error)));
    }

    if let Ok(value) = sqlx::types::time::OffsetDateTime::parse(value, &Rfc3339) {
        return Ok(value);
    }

    for pattern in [
        "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond][offset_hour sign:mandatory]",
        "[year]-[month]-[day] [hour]:[minute]:[second][offset_hour sign:mandatory]",
        "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond][offset_hour sign:mandatory]:[offset_minute]",
        "[year]-[month]-[day] [hour]:[minute]:[second][offset_hour sign:mandatory]:[offset_minute]",
    ] {
        let format = time::format_description::parse(pattern)
            .map_err(|error| sqlx::Error::Decode(Box::new(error)))?;

        if let Ok(value) = OffsetDateTime::parse(value.trim(), &format) {
            return Ok(value);
        }
    }

    for pattern in [
        "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond]",
        "[year]-[month]-[day] [hour]:[minute]:[second]",
    ] {
        let format = time::format_description::parse(pattern)
            .map_err(|error| sqlx::Error::Decode(Box::new(error)))?;

        if let Ok(value) = PrimitiveDateTime::parse(value.trim(), &format) {
            return Ok(value.assume_offset(UtcOffset::UTC));
        }
    }

    Err(sqlx::Error::Decode("unsupported timestamp format".into()))
}

pub mod agent_data;
pub mod auth;
pub mod kill;
pub mod profile;
pub mod similarity;
pub mod user;

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
    #[error("internal")]
    Internal(String),
    #[cfg_attr(feature = "telegram-auth", allow(dead_code))]
    #[error("password hash error")]
    PasswordHash,
    #[error("token error")]
    Token,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let mut message = self.to_string();
        let status = match self {
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::Forbidden => StatusCode::FORBIDDEN,
            Self::BadRequest(msg) => {
                message = msg;
                StatusCode::BAD_REQUEST
            }
            Self::PasswordHash | Self::Token => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Internal(msg) => {
                message = msg;
                StatusCode::INTERNAL_SERVER_ERROR
            }
            Self::Database(msg) => {
                message = msg.to_string();
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };

        let body = Json(json!({ "error": message }));
        (status, body).into_response()
    }
}

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
}

pub fn parse_uuid(row: &AnyRow, column: &str) -> Result<Uuid, sqlx::Error> {
    let value: String = row.try_get(column)?;
    Uuid::parse_str(&value).map_err(|error| sqlx::Error::Decode(Box::new(error)))
}

pub fn parse_optional_uuid(row: &AnyRow, column: &str) -> Result<Option<Uuid>, sqlx::Error> {
    let value: Option<String> = row.try_get(column)?;
    match value {
        Some(value) => Uuid::parse_str(&value)
            .map(Some)
            .map_err(|error| sqlx::Error::Decode(Box::new(error))),
        None => Ok(None),
    }
}

pub fn parse_json(row: &AnyRow, column: &str) -> Result<serde_json::Value, sqlx::Error> {
    let value: String = row.try_get(column)?;
    serde_json::from_str(&value).map_err(|error| sqlx::Error::Decode(Box::new(error)))
}

pub fn parse_optional_timestamp(
    row: &AnyRow,
    column: &str,
) -> Result<Option<sqlx::types::time::OffsetDateTime>, sqlx::Error> {
    let value: Option<String> = row.try_get(column)?;
    value
        .map(|value| decode_timestamp_value(&value))
        .transpose()
}

pub fn parse_timestamp(
    row: &AnyRow,
    column: &str,
) -> Result<sqlx::types::time::OffsetDateTime, sqlx::Error> {
    let value: String = row.try_get(column)?;
    decode_timestamp_value(&value)
}

pub fn db_uuid(value: Uuid) -> String {
    value.to_string()
}

pub fn db_optional_uuid(value: Option<Uuid>) -> Option<String> {
    value.map(|value| value.to_string())
}

pub fn db_json(value: &serde_json::Value) -> String {
    value.to_string()
}

pub fn db_optional_json(value: Option<&serde_json::Value>) -> Option<String> {
    value.map(|value| value.to_string())
}

pub fn db_optional_timestamp(value: Option<sqlx::types::time::OffsetDateTime>) -> Option<String> {
    value
        .map(|value| value.format(&time::format_description::well_known::Rfc3339))
        .transpose()
        .expect("rfc3339 timestamp")
}
