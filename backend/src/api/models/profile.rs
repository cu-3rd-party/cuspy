use crate::api::models::{parse_json, parse_optional_timestamp, parse_timestamp, parse_uuid};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{FromRow, Row, any::AnyRow};
use uuid::Uuid;

pub struct ProfileRequestRecord {
    pub profile_request_id: Uuid,
    pub user_id: Uuid,
    pub requested_profile_data_id: Uuid,
    pub status: String,
    pub reviewer_note: Option<String>,
    pub reviewed_at: Option<sqlx::types::time::OffsetDateTime>,
    pub created_at: sqlx::types::time::OffsetDateTime,
    pub updated_at: sqlx::types::time::OffsetDateTime,
}

impl<'r> FromRow<'r, AnyRow> for ProfileRequestRecord {
    fn from_row(row: &'r AnyRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            profile_request_id: parse_uuid(row, "profile_request_id")?,
            user_id: parse_uuid(row, "user_id")?,
            requested_profile_data_id: parse_uuid(row, "requested_profile_data_id")?,
            status: row.try_get("status")?,
            reviewer_note: row.try_get("reviewer_note")?,
            reviewed_at: parse_optional_timestamp(row, "reviewed_at")?,
            created_at: parse_timestamp(row, "created_at")?,
            updated_at: parse_timestamp(row, "updated_at")?,
        })
    }
}

#[derive(Serialize)]
pub struct ProfileRequestResponse {
    pub profile_request_id: Uuid,
    pub user_id: Uuid,
    pub requested_profile_data_id: Uuid,
    pub status: String,
    pub reviewer_note: Option<String>,
    pub reviewed_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Deserialize)]
pub struct CreateProfileRequest {
    pub requested_profile_data: Value,
}

#[derive(Deserialize)]
pub struct UpdateProfileRequest {
    pub requested_profile_data: Option<Value>,
}

#[derive(Deserialize)]
pub struct AdminUpdateProfileRequest {
    pub requested_profile_data: Option<Value>,
    pub status: Option<String>,
    pub reviewer_note: Option<String>,
}
