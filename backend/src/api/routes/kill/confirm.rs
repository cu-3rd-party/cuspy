use crate::api::extractor::AuthUser;
use crate::api::models::kill::{ConfirmKillRequest, KillEventRecord, KillEventResponse};
use crate::api::models::{ApiError, db_uuid, kill};
use crate::api::routes;
use crate::api::routes::kill::helpers::KILL_EVENT_COLUMNS;
use crate::{ApiContext, notifier};
use axum::Json;
use axum::extract::{Path, State};
use uuid::Uuid;

pub async fn confirm_kill(
    State(state): State<ApiContext>,
    AuthUser(user): AuthUser,
    Path(kill_id): Path<Uuid>,
    Json(payload): Json<ConfirmKillRequest>,
) -> Result<Json<KillEventResponse>, ApiError> {
    let kill = routes::kill::helpers::fetch_kill(&state, kill_id).await?;

    if user.user_id != kill.killer_id && user.user_id != kill.victim_id {
        return Err(ApiError::Forbidden);
    }

    let note = payload
        .note
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty());

    let other_user_id = if user.user_id == kill.killer_id {
        kill.victim_id
    } else {
        kill.killer_id
    };

    let approve_query = format!(
        r#"
        update kill_event
        set
            killer_confirmed_at = case
                when cast(killer_id as text) = $2 then coalesce(killer_confirmed_at, now())
                else killer_confirmed_at
            end,
            victim_confirmed_at = case
                when cast(victim_id as text) = $2 then coalesce(victim_confirmed_at, now())
                else victim_confirmed_at
            end,
            status = case
                when (
                    case
                        when cast(killer_id as text) = $2 then coalesce(killer_confirmed_at, now())
                        else killer_confirmed_at
                    end
                ) is not null
                and (
                    case
                        when cast(victim_id as text) = $2 then coalesce(victim_confirmed_at, now())
                        else victim_confirmed_at
                    end
                ) is not null
                then 'CONFIRMED'
                else 'REPORTED'
            end,
            confirmed_at = case
                when (
                    case
                        when cast(killer_id as text) = $2 then coalesce(killer_confirmed_at, now())
                        else killer_confirmed_at
                    end
                ) is not null
                and (
                    case
                        when cast(victim_id as text) = $2 then coalesce(victim_confirmed_at, now())
                        else victim_confirmed_at
                    end
                ) is not null
                then coalesce(confirmed_at, now())
                else confirmed_at
            end
        where cast(kill_event_id as text) = $1
          and status in ('REPORTED', 'CONFIRMED')
        returning
            {KILL_EVENT_COLUMNS}
        "#
    );

    let reject_query = format!(
        r#"
        update kill_event
        set
            status = 'REJECTED',
            moderation_reason = coalesce($2, moderation_reason),
            moderated_at = now(),
            moderator_id = null
        where cast(kill_event_id as text) = $1
          and status in ('REPORTED', 'CONFIRMED')
        returning
            {KILL_EVENT_COLUMNS}
        "#
    );

    let record = if !payload.confirmed {
        sqlx::query_as::<_, KillEventRecord>(&reject_query)
            .bind(db_uuid(kill_id))
            .bind(note)
            .fetch_optional(&state.db)
            .await?
            .ok_or(ApiError::BadRequest(
                "kill can no longer be rejected".into(),
            ))?
    } else {
        sqlx::query_as::<_, KillEventRecord>(&approve_query)
            .bind(db_uuid(kill_id))
            .bind(db_uuid(user.user_id))
            .fetch_optional(&state.db)
            .await?
            .ok_or(ApiError::BadRequest(
                "kill can no longer be confirmed".into(),
            ))?
    };

    if !payload.confirmed {
        notifier::notify_user(
            &state,
            other_user_id,
            format!(
                "Kill report {} rejected by counterparty. Review notes in system.",
                record.kill_event_id
            ),
        )
        .await;
    } else if kill.status != "CONFIRMED" && record.status == "CONFIRMED" {
        notifier::notify_user(
            &state,
            other_user_id,
            format!(
                "Kill report {} confirmed by both participants. Awaiting admin moderation.",
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
