use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct ReportKillRequest {
    pub victim_id: Uuid,
    pub evidence_url: Option<String>,
    pub details: Option<Value>,
}

#[derive(Deserialize)]
pub struct ConfirmKillRequest {
    pub confirmed: bool,
    pub note: Option<String>,
}

#[derive(Deserialize)]
pub struct ModerateKillRequest {
    pub action: String,
    pub reason: Option<String>,
}

#[derive(Serialize, FromRow)]
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

#[derive(Serialize, FromRow)]
pub struct RankingEntry {
    pub rank: i64,
    pub user_id: Uuid,
    pub agent_name: Option<String>,
    pub rating: i64,
    pub approved_kills: i64,
    pub approved_deaths: i64,
}

#[derive(Serialize, FromRow)]
pub struct UserStatsResponse {
    pub user_id: Uuid,
    pub rating: i64,
    pub approved_kills: i64,
    pub approved_deaths: i64,
    pub pending_kills: i64,
}
