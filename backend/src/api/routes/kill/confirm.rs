use crate::api::models::kill::{ConfirmKillRequest, KillEventRecord, KillEventResponse};
use crate::api::models::{ApiError, db_uuid, kill};
use crate::api::{helpers, routes};
use crate::{ApiContext, notifier};
use axum::Json;
use axum::extract::{Path, State};
use http::HeaderMap;
use uuid::Uuid;

pub async fn confirm_kill(
    State(state): State<ApiContext>,
    headers: HeaderMap,
    Path(kill_id): Path<Uuid>,
    Json(payload): Json<ConfirmKillRequest>,
) -> Result<Json<KillEventResponse>, ApiError> {
    let auth = helpers::require_bearer_token(&headers, &state)?;
    let kill = routes::kill::helpers::fetch_kill(&state, kill_id).await?;

    if auth.user_id != kill.killer_id && auth.user_id != kill.victim_id {
        return Err(ApiError::Forbidden);
    }

    let note = payload
        .note
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty());

    let record = if !payload.confirmed {
        sqlx::query_as::<_, KillEventRecord>(
            r#"
            update kill_event
            set
                status = 'REJECTED',
                moderation_reason = coalesce($2, moderation_reason),
                moderated_at = now(),
                moderator_id = null
            where cast(kill_event_id as text) = $1
              and status in ('REPORTED', 'VICTIM_CONFIRMED')
            returning
                cast(kill_event_id as text) as kill_event_id,
                cast(killer_id as text) as killer_id,
                cast(victim_id as text) as victim_id,
                status,
                evidence_url, // TODO: refactor into the new resource location logic
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
        .bind(note)
        .fetch_optional(&state.db)
        .await?
        .ok_or(ApiError::BadRequest(
            "kill can no longer be rejected".into(),
        ))?
    } else if auth.user_id == kill.killer_id {
        sqlx::query_as::<_, KillEventRecord>(
            r#"
            update kill_event
            set killer_confirmed_at = coalesce(killer_confirmed_at, now())
            where cast(kill_event_id as text) = $1
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
        .fetch_one(&state.db)
        .await?
    } else {
        sqlx::query_as::<_, KillEventRecord>(
            r#"
            update kill_event
            set
                victim_confirmed_at = coalesce(victim_confirmed_at, now()),
                confirmed_at = coalesce(confirmed_at, now()),
                status = case when status = 'REPORTED' then 'VICTIM_CONFIRMED' else status end
            where cast(kill_event_id as text) = $1
              and status in ('REPORTED', 'VICTIM_CONFIRMED')
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
        .fetch_optional(&state.db)
        .await?
        .ok_or(ApiError::BadRequest(
            "kill can no longer be confirmed".into(),
        ))?
    };

    if !payload.confirmed {
        notifier::notify_user(
            &state,
            kill.killer_id,
            format!(
                "Kill report {} rejected by counterparty. Review notes in system.",
                record.kill_event_id
            ),
        )
        .await;
    } else if auth.user_id == kill.victim_id {
        notifier::notify_user(
            &state,
            kill.killer_id,
            format!(
                "Victim confirmed kill report {}. Awaiting admin moderation.",
                record.kill_event_id
            ),
        )
        .await;
        notifier::notify_admins(
            &state,
            format!(
                "Kill report {} confirmed by victim and ready for moderation.",
                record.kill_event_id
            ),
        )
        .await;
    }

    Ok(Json(kill::to_kill_response(record)))
}
