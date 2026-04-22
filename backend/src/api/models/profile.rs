use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
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
