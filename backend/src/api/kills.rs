use axum::{
    Json,
    extract::{Path, State},
};
use http::HeaderMap;
use serde_json::{Value, json};
use sqlx::FromRow;
use uuid::Uuid;

use crate::{
    AppState,
    api::{
        helpers,
        models::{db_json, db_uuid, parse_json, parse_optional_timestamp, parse_optional_uuid, parse_timestamp, parse_uuid},
        models::{
            ApiError,
            kill::{
                ConfirmKillRequest, KillEventResponse, ModerateKillRequest, RankingEntry,
                ReportKillRequest, UserStatsResponse,
            },
        },
    },
};
use crate::notifier;

fn format_timestamp(value: sqlx::types::time::OffsetDateTime) -> String {
    value.unix_timestamp().to_string()
}

struct KillEventRecord {
    kill_event_id: Uuid,
    killer_id: Uuid,
    victim_id: Uuid,
    status: String,
    evidence_url: Option<String>,
    details: Value,
    killer_confirmed_at: Option<sqlx::types::time::OffsetDateTime>,
    victim_confirmed_at: Option<sqlx::types::time::OffsetDateTime>,
    moderation_reason: Option<String>,
    reported_at: sqlx::types::time::OffsetDateTime,
    confirmed_at: Option<sqlx::types::time::OffsetDateTime>,
    moderated_at: Option<sqlx::types::time::OffsetDateTime>,
    moderator_id: Option<Uuid>,
    created_at: sqlx::types::time::OffsetDateTime,
    updated_at: Option<sqlx::types::time::OffsetDateTime>,
}

impl<'r> FromRow<'r, sqlx::any::AnyRow> for KillEventRecord {
    fn from_row(row: &'r sqlx::any::AnyRow) -> Result<Self, sqlx::Error> {
        use sqlx::Row;

        Ok(Self {
            kill_event_id: parse_uuid(row, "kill_event_id")?,
            killer_id: parse_uuid(row, "killer_id")?,
            victim_id: parse_uuid(row, "victim_id")?,
            status: row.try_get("status")?,
            evidence_url: row.try_get("evidence_url")?,
            details: parse_json(row, "details")?,
            killer_confirmed_at: parse_optional_timestamp(row, "killer_confirmed_at")?,
            victim_confirmed_at: parse_optional_timestamp(row, "victim_confirmed_at")?,
            moderation_reason: row.try_get("moderation_reason")?,
            reported_at: parse_timestamp(row, "reported_at")?,
            confirmed_at: parse_optional_timestamp(row, "confirmed_at")?,
            moderated_at: parse_optional_timestamp(row, "moderated_at")?,
            moderator_id: parse_optional_uuid(row, "moderator_id")?,
            created_at: parse_timestamp(row, "created_at")?,
            updated_at: parse_optional_timestamp(row, "updated_at")?,
        })
    }
}

fn to_kill_response(record: KillEventRecord) -> KillEventResponse {
    KillEventResponse {
        kill_event_id: record.kill_event_id,
        killer_id: record.killer_id,
        victim_id: record.victim_id,
        status: record.status,
        evidence_url: record.evidence_url,
        details: record.details,
        killer_confirmed_at: record.killer_confirmed_at.map(format_timestamp),
        victim_confirmed_at: record.victim_confirmed_at.map(format_timestamp),
        moderation_reason: record.moderation_reason,
        reported_at: format_timestamp(record.reported_at),
        confirmed_at: record.confirmed_at.map(format_timestamp),
        moderated_at: record.moderated_at.map(format_timestamp),
        moderator_id: record.moderator_id,
        created_at: format_timestamp(record.created_at),
        updated_at: record.updated_at.map(format_timestamp),
    }
}

async fn fetch_kill(state: &AppState, kill_id: Uuid) -> Result<KillEventRecord, ApiError> {
    sqlx::query_as::<_, KillEventRecord>(state.db_param(
        r#"
        select
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
        from kill_event
        where kill_event_id = $1
        "#,
        r#"
        select
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
        from kill_event
        where cast(kill_event_id as text) = $1
        "#,
    ))
    .bind(db_uuid(kill_id))
    .fetch_optional(&state.db)
    .await?
    .ok_or(ApiError::NotFound)
}

pub async fn report_kill(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<ReportKillRequest>,
) -> Result<(http::StatusCode, Json<KillEventResponse>), ApiError> {
    let auth = helpers::require_bearer_token(&headers, &state)?;
    if auth.user_id == payload.victim_id {
        return Err(ApiError::BadRequest("killer and victim must differ".into()));
    }

    let details = match payload.details {
        Some(Value::Object(map)) => Value::Object(map),
        Some(_) => {
            return Err(ApiError::BadRequest(
                "kill details must be a JSON object".into(),
            ));
        }
        None => json!({}),
    };

    let record = sqlx::query_as::<_, KillEventRecord>(state.db_param(
        r#"
        insert into kill_event (
            kill_event_id,
            killer_id,
            victim_id,
            evidence_url,
            details,
            killer_confirmed_at
        )
        values ($1, $2, $3, $4, $5, now())
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
        r#"
        insert into kill_event (
            kill_event_id,
            killer_id,
            victim_id,
            evidence_url,
            details,
            killer_confirmed_at
        )
        values (cast($1 as uuid), cast($2 as uuid), cast($3 as uuid), $4, cast($5 as jsonb), now())
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
    ))
    .bind(db_uuid(Uuid::now_v7()))
    .bind(db_uuid(auth.user_id))
    .bind(db_uuid(payload.victim_id))
    .bind(payload.evidence_url)
    .bind(db_json(&details))
    .fetch_one(&state.db)
    .await?;

    notifier::notify_user(
        &state,
        payload.victim_id,
        format!(
            "Kill report filed against you. Open reveal confirmation. Reference: {}",
            record.kill_event_id
        ),
    )
    .await;
    notifier::notify_admins(
        &state,
        format!("Kill report {} created and awaiting victim response.", record.kill_event_id),
    )
    .await;

    Ok((http::StatusCode::CREATED, Json(to_kill_response(record))))
}

pub async fn confirm_kill(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(kill_id): Path<Uuid>,
    Json(payload): Json<ConfirmKillRequest>,
) -> Result<Json<KillEventResponse>, ApiError> {
    let auth = helpers::require_bearer_token(&headers, &state)?;
    let kill = fetch_kill(&state, kill_id).await?;

    if auth.user_id != kill.killer_id && auth.user_id != kill.victim_id {
        return Err(ApiError::Forbidden);
    }

    let note = payload
        .note
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty());

    let record = if !payload.confirmed {
        sqlx::query_as::<_, KillEventRecord>(state.db_param(
            r#"
            update kill_event
            set
                status = 'REJECTED',
                moderation_reason = coalesce($2, moderation_reason),
                moderated_at = now(),
                moderator_id = null
            where kill_event_id = $1
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
        ))
        .bind(db_uuid(kill_id))
        .bind(note)
        .fetch_optional(&state.db)
        .await?
        .ok_or(ApiError::BadRequest(
            "kill can no longer be rejected".into(),
        ))?
    } else if auth.user_id == kill.killer_id {
        sqlx::query_as::<_, KillEventRecord>(state.db_param(
            r#"
            update kill_event
            set killer_confirmed_at = coalesce(killer_confirmed_at, now())
            where kill_event_id = $1
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
        ))
        .bind(db_uuid(kill_id))
        .fetch_one(&state.db)
        .await?
    } else {
        sqlx::query_as::<_, KillEventRecord>(state.db_param(
            r#"
            update kill_event
            set
                victim_confirmed_at = coalesce(victim_confirmed_at, now()),
                confirmed_at = coalesce(confirmed_at, now()),
                status = case when status = 'REPORTED' then 'VICTIM_CONFIRMED' else status end
            where kill_event_id = $1
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
        ))
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

    Ok(Json(to_kill_response(record)))
}

pub async fn moderate_kill(
    State(state): State<AppState>,
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

    let record = sqlx::query_as::<_, KillEventRecord>(state.db_param(
        r#"
        update kill_event
        set
            status = $2,
            moderation_reason = $3,
            moderated_at = now(),
            moderator_id = nullif($4, $5)
        where kill_event_id = $1
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
    ))
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
                format!("Kill report {} approved. Rating changes applied.", record.kill_event_id),
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
                format!("Kill report {} rejected. Reason: {reason}", record.kill_event_id),
            )
            .await;
            notifier::notify_user(
                &state,
                record.victim_id,
                format!("Kill report {} rejected by admin. Reason: {reason}", record.kill_event_id),
            )
            .await;
        }
        _ => {}
    }

    Ok(Json(to_kill_response(record)))
}

pub async fn list_my_pending_kills(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Vec<KillEventResponse>>, ApiError> {
    let auth = helpers::require_bearer_token(&headers, &state)?;
    let records = sqlx::query_as::<_, KillEventRecord>(state.db_param(
        r#"
        select
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
        from kill_event
        where status in ('REPORTED', 'VICTIM_CONFIRMED')
          and (
              (cast(killer_id as text) = $1 and killer_confirmed_at is null)
              or (cast(victim_id as text) = $1 and victim_confirmed_at is null)
          )
        order by reported_at desc, created_at desc
        "#,
        r#"
        select
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
        from kill_event
        where status in ('REPORTED', 'VICTIM_CONFIRMED')
          and (
              (cast(killer_id as text) = $1 and killer_confirmed_at is null)
              or (cast(victim_id as text) = $1 and victim_confirmed_at is null)
          )
        order by reported_at desc, created_at desc
        "#,
    ))
    .bind(db_uuid(auth.user_id))
    .fetch_all(&state.db)
    .await?;

    Ok(Json(records.into_iter().map(to_kill_response).collect()))
}

pub async fn list_approved_kills(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Vec<KillEventResponse>>, ApiError> {
    let _auth = helpers::require_bearer_token(&headers, &state)?;
    let records = sqlx::query_as::<_, KillEventRecord>(
        r#"
        select
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
        from kill_event
        where status = 'ADMIN_APPROVED'
        order by moderated_at desc nulls last, created_at desc
        "#,
    )
    .fetch_all(&state.db)
    .await?;

    Ok(Json(records.into_iter().map(to_kill_response).collect()))
}

pub async fn rankings(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Vec<RankingEntry>>, ApiError> {
    let _auth = helpers::require_bearer_token(&headers, &state)?;
    let entries = sqlx::query_as::<_, RankingEntry>(
        r#"
        with latest_ratings as (
            select distinct on (user_id) user_id, rating
            from rating_history
            order by user_id, created_at desc, rating_history_id desc
        ),
        kill_totals as (
            select
                user_id,
                coalesce(sum(approved_kills), 0)::bigint as approved_kills,
                coalesce(sum(approved_deaths), 0)::bigint as approved_deaths
            from (
                select killer_id as user_id, count(*)::bigint as approved_kills, 0::bigint as approved_deaths
                from kill_event
                where status = 'ADMIN_APPROVED'
                group by killer_id
                union all
                select victim_id as user_id, 0::bigint as approved_kills, count(*)::bigint as approved_deaths
                from kill_event
                where status = 'ADMIN_APPROVED'
                group by victim_id
            ) totals
                group by user_id
        ),
        leaderboard as (
        select
            rank() over (order by coalesce(latest_ratings.rating, $1) desc, u.created_at asc)::bigint as rank,
            u.user_id,
            u.agent_name,
            coalesce(latest_ratings.rating, $1) as rating,
            coalesce(kill_totals.approved_kills, 0) as approved_kills,
            coalesce(kill_totals.approved_deaths, 0) as approved_deaths
        from "user" u
        left join latest_ratings on latest_ratings.user_id = u.user_id
        left join kill_totals on kill_totals.user_id = u.user_id
        )
        select rank, cast(user_id as text) as user_id, agent_name, rating, approved_kills, approved_deaths
        from leaderboard
        order by rank asc, user_id asc
        "#,
    )
    .bind(helpers::DEFAULT_RATING)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(entries))
}

pub async fn user_stats(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(user_id): Path<Uuid>,
) -> Result<Json<UserStatsResponse>, ApiError> {
    let _auth = helpers::require_bearer_token(&headers, &state)?;
    let stats = sqlx::query_as::<_, UserStatsResponse>(
        r#"
        with latest_rating as (
            select rating
            from rating_history
            where cast(user_id as text) = $1
            order by created_at desc, rating_history_id desc
            limit 1
        )
        select
            cast($1 as text) as user_id,
            coalesce((select rating from latest_rating), $2) as rating,
            (
                select count(*)::bigint
                from kill_event
                where cast(killer_id as text) = $1 and status = 'ADMIN_APPROVED'
            ) as approved_kills,
            (
                select count(*)::bigint
                from kill_event
                where cast(victim_id as text) = $1 and status = 'ADMIN_APPROVED'
            ) as approved_deaths,
            (
                select count(*)::bigint
                from kill_event
                where cast(killer_id as text) = $1 and status in ('REPORTED', 'VICTIM_CONFIRMED')
            ) as pending_kills
        where exists (select 1 from "user" where cast(user_id as text) = $1)
        "#,
    )
    .bind(db_uuid(user_id))
    .bind(helpers::DEFAULT_RATING)
    .fetch_optional(&state.db)
    .await?
    .ok_or(ApiError::NotFound)?;

    Ok(Json(stats))
}
