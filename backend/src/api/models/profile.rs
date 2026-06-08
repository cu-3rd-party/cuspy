use crate::api::models::{parse_json, parse_optional_timestamp, parse_timestamp, parse_uuid};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{FromRow, Row, any::AnyRow};
use uuid::Uuid;

pub struct ProfileCreationRequestRecord {
    pub profile_creation_request_id: Uuid,
    pub user_id: Uuid,
    pub requested_profile_data: Value,
    pub status: String,
    pub reviewer_note: Option<String>,
    pub reviewed_at: Option<sqlx::types::time::OffsetDateTime>,
    pub created_at: sqlx::types::time::OffsetDateTime,
    pub updated_at: sqlx::types::time::OffsetDateTime,
}

impl<'r> FromRow<'r, AnyRow> for ProfileCreationRequestRecord {
    fn from_row(row: &'r AnyRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            profile_creation_request_id: parse_uuid(row, "profile_creation_request_id")?,
            user_id: parse_uuid(row, "user_id")?,
            requested_profile_data: parse_json(row, "requested_profile_data")?,
            status: row.try_get("status")?,
            reviewer_note: row.try_get("reviewer_note")?,
            reviewed_at: parse_optional_timestamp(row, "reviewed_at")?,
            created_at: parse_timestamp(row, "created_at")?,
            updated_at: parse_timestamp(row, "updated_at")?,
        })
    }
}

#[derive(Serialize)]
pub struct ProfileCreationRequestResponse {
    pub profile_creation_request_id: Uuid,
    pub user_id: Uuid,
    pub requested_profile_data: Value,
    pub status: String,
    pub reviewer_note: Option<String>,
    pub reviewed_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Deserialize)]
pub struct CreateProfileCreationRequest {
    pub requested_profile_data: Value,
}

#[derive(Deserialize)]
pub struct UpdateProfileCreationRequest {
    pub requested_profile_data: Option<Value>,
}

#[derive(Deserialize)]
pub struct AdminUpdateProfileCreationRequest {
    pub requested_profile_data: Option<Value>,
    pub status: Option<String>,
    pub reviewer_note: Option<String>,
}
