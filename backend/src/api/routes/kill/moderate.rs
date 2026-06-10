use crate::api::helpers;
use crate::api::models::kill::{KillEventRecord, KillEventResponse, ModerateKillRequest};
use crate::api::models::{ApiError, db_uuid, kill};
use crate::{ApiContext, notifier};
use axum::Json;
use axum::extract::{Path, State};
use http::HeaderMap;
use uuid::Uuid;

pub async fn moderate_kill(
    State(state): State<ApiContext>,
    headers: HeaderMap,
    Path(kill_id): Path<Uuid>,
    Json(payload): Json<ModerateKillRequest>,
) -> Result<Json<KillEventResponse>, ApiError> {
    let admin = helpers::require_admin(&headers, &state)?;
    let action = payload.action.trim();
    let reason = payload
        .reason
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty());

    let (next_status, required_state) = match action {
        "APPROVE" => ("ADMIN_APPROVED", "VICTIM_CONFIRMED"),
        "REJECT" => ("REJECTED", "REPORTED"),
        _ => return Err(ApiError::BadRequest("invalid moderation action".into())),
    };

    let record = sqlx::query_as::<_, KillEventRecord>(
        r#"
        update kill_event
        set
            status = $2,
            moderation_reason = $3,
            moderated_at = now(),
            moderator_id = nullif(cast($4 as uuid), cast($5 as uuid))
        where cast(kill_event_id as text) = $1
          and (
              ($2 = 'ADMIN_APPROVED' and status = 'VICTIM_CONFIRMED')
              or ($2 = 'REJECTED' and status in ('REPORTED', 'VICTIM_CONFIRMED'))
          )
        returning
            cast(kill_event_id as text) as kill_event_id,
            cast(killer_id as text) as killer_id,
            cast(victim_id as text) as victim_id,
            status,
            evidence_url,
            cast(details as text) as details,
            cast(killer_confirmed_at as text) as killer_confirmed_at,
            cast(victim_confirmed_at as text) as victim_confirmed_at,
            moderation_reason,
            cast(reported_at as text) as reported_at,
            cast(confirmed_at as text) as confirmed_at,
            cast(moderated_at as text) as moderated_at,
            cast(moderator_id as text) as moderator_id,
            cast(created_at as text) as created_at,
            cast(updated_at as text) as updated_at
        "#,
    )
    .bind(db_uuid(kill_id))
    .bind(next_status)
    .bind(reason)
    .bind(db_uuid(admin.user_id))
    .bind(db_uuid(Uuid::nil()))
    .fetch_optional(&state.db)
    .await?
    .ok_or(ApiError::BadRequest(format!(
        "kill must be in {required_state} before {action}"
    )))?;

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
