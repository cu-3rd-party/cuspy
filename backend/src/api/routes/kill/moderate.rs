use crate::api::extractor;
use crate::api::models::kill::KillEventResponse;
use crate::api::models::{ApiError, kill};
use crate::api::routes::kill::helpers::fetch_kill;
use crate::{ApiContext, notifier};
use axum::Json;
use axum::extract::{Path, State};
use http::HeaderMap;
use serde::Deserialize;
use uuid::Uuid;
use crate::api::extractor::AdminUser;

#[derive(Deserialize)]
pub enum ModerationActions {
    Approve,
    Reject,
    // а какие еще нужны то
}

#[derive(Deserialize)]
pub struct ModerateKillRequest {
    pub action: ModerationActions,
    pub reason: Option<String>,
}

// Эндпоинт использует админ.
pub async fn moderate_kill(
    State(state): State<ApiContext>,
    AdminUser(_user): AdminUser,
    Path(kill_id): Path<Uuid>,
    Json(request): Json<ModerateKillRequest>,
) -> Result<Json<KillEventResponse>, ApiError> {
    let reason = request
        .reason
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty());

    // TODO: оно должно внутри одной транзакции читать состояние и обновлять его в соответствии с действием админа
    let record = fetch_kill(&state, kill_id).await?;

    match request.action {
        ModerationActions::Approve => {}
        ModerationActions::Reject => {}
    }

    // legacy code part for reference
    match record.status.as_str() {
        "ADMIN_APPROVED" => {
            notifier::notify_user(
                &state,
                record.killer_id,
                format!(
                    "Kill report {} approved. Rating changes applied.",
                    record.kill_event_id
                ),
            )
            .await;
            notifier::notify_user(
                &state,
                record.victim_id,
                format!("Kill report {} approved by admin.", record.kill_event_id),
            )
            .await;
        }
        "REJECTED" => {
            let reason = record
                .moderation_reason
                .clone()
                .unwrap_or_else(|| "No reason attached.".to_string());
            notifier::notify_user(
                &state,
                record.killer_id,
                format!(
                    "Kill report {} rejected. Reason: {reason}",
                    record.kill_event_id
                ),
            )
            .await;
            notifier::notify_user(
                &state,
                record.victim_id,
                format!(
                    "Kill report {} rejected by admin. Reason: {reason}",
                    record.kill_event_id
                ),
            )
            .await;
        }
        _ => {}
    }

    Ok(Json(kill::to_kill_response(record)))
}
