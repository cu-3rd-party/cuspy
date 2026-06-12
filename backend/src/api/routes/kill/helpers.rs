use crate::ApiContext;
use crate::api::models::kill::KillEventRecord;
use crate::api::models::{ApiError, db_json, db_optional_timestamp, db_optional_uuid, db_uuid};
use uuid::Uuid;

pub const KILL_EVENT_COLUMNS: &str = r#"
    cast(kill_event_id as text) as kill_event_id,
    cast(killer_id as text) as killer_id,
    cast(victim_id as text) as victim_id,
    status,
    cast(evidence_resource_id as text) as evidence_resource_id,
    cast(details as text) as details,
    cast(killer_confirmed_at as text) as killer_confirmed_at,
    cast(victim_confirmed_at as text) as victim_confirmed_at,
    cast(created_at as text) as reported_at,
    cast(confirmed_at as text) as confirmed_at,
    cast(moderated_at as text) as moderated_at,
    cast(moderator_id as text) as moderator_id,
    moderation_reason,
    cast(rating_applied_at as text) as rating_applied_at,
    cast(created_at as text) as created_at,
    cast(updated_at as text) as updated_at
"#;

pub async fn fetch_kill(state: &ApiContext, kill_id: Uuid) -> Result<KillEventRecord, ApiError> {
    let query = format!(
        r#"
        select
            {KILL_EVENT_COLUMNS}
        from kill_event
        where kill_event_id = $1
        "#
    );

    sqlx::query_as::<_, KillEventRecord>(&query)
        .bind(db_uuid(kill_id))
        .fetch_optional(&state.db)
        .await?
        .ok_or(ApiError::NotFound)
}

#[allow(dead_code)]
pub async fn update_kill(
    state: &ApiContext,
    kill_id: Uuid,
    kill_event_record: KillEventRecord,
) -> Result<KillEventRecord, ApiError> {
    let query = format!(
        r#"
        update kill_event
        set
            killer_id = cast($2 as uuid),
            victim_id = cast($3 as uuid),
            status = $4,
            evidence_resource_id = coalesce(cast($5 as uuid), evidence_resource_id),
            details = cast($6 as jsonb),
            killer_confirmed_at = $7,
            victim_confirmed_at = $8,
            confirmed_at = $9,
            moderated_at = $10,
            moderator_id = cast($11 as uuid),
            moderation_reason = $12,
            rating_applied_at = $13
        where kill_event_id = cast($1 as uuid)
        returning
            {KILL_EVENT_COLUMNS}
        "#
    );

    sqlx::query_as::<_, KillEventRecord>(&query)
        .bind(db_uuid(kill_id))
        .bind(db_uuid(kill_event_record.killer_id))
        .bind(db_uuid(kill_event_record.victim_id))
        .bind(&kill_event_record.status)
        .bind(db_optional_uuid(kill_event_record.evidence_resource_id))
        .bind(db_json(&kill_event_record.details))
        .bind(db_optional_timestamp(kill_event_record.killer_confirmed_at))
        .bind(db_optional_timestamp(kill_event_record.victim_confirmed_at))
        .bind(db_optional_timestamp(kill_event_record.confirmed_at))
        .bind(db_optional_timestamp(kill_event_record.moderated_at))
        .bind(db_optional_uuid(kill_event_record.moderator_id))
        .bind(&kill_event_record.moderation_reason)
        .bind(db_optional_timestamp(kill_event_record.rating_applied_at))
        .fetch_one(&state.db)
        .await
        .map_err(Into::into)
}
