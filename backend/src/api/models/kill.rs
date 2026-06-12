use crate::api::helpers;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{FromRow, any::AnyRow};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize, ToSchema)]
pub struct ReportKillRequest {
    pub killer_id: Option<Uuid>,
    pub victim_id: Option<Uuid>,
    pub evidence_url: Option<String>,
    pub details: Option<Value>,
}

#[derive(Deserialize, ToSchema)]
pub struct ConfirmKillRequest {
    pub confirmed: bool,
    pub note: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct KillEventResponse {
    pub kill_event_id: Uuid,
    pub killer_id: Uuid,
    pub victim_id: Uuid,
    pub status: String,
    pub evidence_url: Option<String>,
    pub details: Value,
    pub killer_confirmed_at: Option<String>,
    pub victim_confirmed_at: Option<String>,
    pub moderation_reason: Option<String>,
    pub reported_at: String,
    pub confirmed_at: Option<String>,
    pub moderated_at: Option<String>,
    pub moderator_id: Option<Uuid>,
    pub created_at: String,
    pub updated_at: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct RankingEntry {
    pub rank: i64,
    pub user_id: Uuid,
    pub agent_name: Option<String>,
    pub rating: i64,
    pub approved_kills: i64,
    pub approved_deaths: i64,
}

#[derive(Serialize, ToSchema)]
pub struct UserStatsResponse {
    pub user_id: Uuid,
    pub rating: i64,
    pub approved_kills: i64,
    pub approved_deaths: i64,
    pub pending_kills: i64,
}

impl<'r> FromRow<'r, AnyRow> for RankingEntry {
    fn from_row(row: &'r AnyRow) -> Result<Self, sqlx::Error> {
        use crate::api::models::parse_uuid;
        use sqlx::Row;

        Ok(Self {
            rank: row.get("rank"),
            user_id: parse_uuid(row, "user_id")?,
            agent_name: row.try_get("agent_name").ok(),
            rating: row.get("rating"),
            approved_kills: row.get("approved_kills"),
            approved_deaths: row.get("approved_deaths"),
        })
    }
}

impl<'r> FromRow<'r, AnyRow> for UserStatsResponse {
    fn from_row(row: &'r AnyRow) -> Result<Self, sqlx::Error> {
        use crate::api::models::parse_uuid;
        use sqlx::Row;

        Ok(Self {
            user_id: parse_uuid(row, "user_id")?,
            rating: row.get("rating"),
            approved_kills: row.get("approved_kills"),
            approved_deaths: row.get("approved_deaths"),
            pending_kills: row.get("pending_kills"),
        })
    }
}

pub struct KillEventRecord {
    pub kill_event_id: Uuid,
    pub killer_id: Uuid,
    pub victim_id: Uuid,
    pub status: String,
    pub evidence_resource_id: Option<Uuid>,
    pub details: Value,
    pub killer_confirmed_at: Option<time::OffsetDateTime>,
    pub victim_confirmed_at: Option<time::OffsetDateTime>,
    pub reported_at: time::OffsetDateTime,
    pub confirmed_at: Option<time::OffsetDateTime>,
    pub moderated_at: Option<time::OffsetDateTime>,
    pub moderator_id: Option<Uuid>,
    pub moderation_reason: Option<String>,
    pub rating_applied_at: Option<time::OffsetDateTime>,
    pub created_at: time::OffsetDateTime,
    pub updated_at: Option<time::OffsetDateTime>,
}

impl<'r> FromRow<'r, sqlx::any::AnyRow> for KillEventRecord {
    fn from_row(row: &'r sqlx::any::AnyRow) -> Result<Self, sqlx::Error> {
        use crate::api::models::{
            parse_json, parse_optional_timestamp, parse_optional_uuid, parse_timestamp, parse_uuid,
        };
        use sqlx::Row;

        Ok(Self {
            kill_event_id: parse_uuid(row, "kill_event_id")?,
            killer_id: parse_uuid(row, "killer_id")?,
            victim_id: parse_uuid(row, "victim_id")?,
            status: row.get("status"),
            evidence_resource_id: parse_optional_uuid(row, "evidence_resource_id").ok().flatten(),
            details: parse_json(row, "details")?,
            killer_confirmed_at: parse_optional_timestamp(row, "killer_confirmed_at").ok().flatten(),
            victim_confirmed_at: parse_optional_timestamp(row, "victim_confirmed_at").ok().flatten(),
            reported_at: parse_timestamp(row, "reported_at")?,
            confirmed_at: parse_optional_timestamp(row, "confirmed_at").ok().flatten(),
            moderated_at: parse_optional_timestamp(row, "moderated_at").ok().flatten(),
            moderator_id: parse_optional_uuid(row, "moderator_id").ok().flatten(),
            moderation_reason: row.try_get("moderation_reason").ok(),
            rating_applied_at: parse_optional_timestamp(row, "rating_applied_at").ok().flatten(),
            created_at: parse_timestamp(row, "created_at")?,
            updated_at: parse_optional_timestamp(row, "updated_at").ok().flatten(),
        })
    }
}

pub fn to_kill_response(record: KillEventRecord) -> KillEventResponse {
    let evidence_url = record
        .details
        .as_object()
        .and_then(|details| details.get("evidence_url"))
        .and_then(Value::as_str)
        .map(str::to_owned);

    KillEventResponse {
        kill_event_id: record.kill_event_id,
        killer_id: record.killer_id,
        victim_id: record.victim_id,
        status: record.status,
        evidence_url,
        details: record.details,
        killer_confirmed_at: record.killer_confirmed_at.map(helpers::format_timestamp),
        victim_confirmed_at: record.victim_confirmed_at.map(helpers::format_timestamp),
        moderation_reason: record.moderation_reason,
        reported_at: helpers::format_timestamp(record.reported_at),
        confirmed_at: record.confirmed_at.map(helpers::format_timestamp),
        moderated_at: record.moderated_at.map(helpers::format_timestamp),
        moderator_id: record.moderator_id,
        created_at: helpers::format_timestamp(record.created_at),
        updated_at: record.updated_at.map(helpers::format_timestamp),
    }
}
