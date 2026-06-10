use crate::ApiContext;
use crate::api::models::kill::KillEventRecord;
use crate::api::models::{ApiError, db_uuid, db_optional_uuid, db_optional_timestamp, db_json, db_optional_json};
use uuid::Uuid;

pub async fn fetch_kill(state: &ApiContext, kill_id: Uuid) -> Result<KillEventRecord, ApiError> {
    sqlx::query_as::<_, KillEventRecord>(
        r#"
        select
            kill_event_id,
            killer_id,
            victim_id,
            status,
            evidence_resource_id,
            details,
            killer_confirmed_at,
            victim_confirmed_at,
            confirmed_at,
            moderated_at,
            moderator_id,
            moderation_reason,
            rating_applied_at,
            created_at,
            updated_at
        from kill_event
        where kill_event_id = $1
        "#,
    )
    .bind(db_uuid(kill_id))
    .fetch_optional(&state.db)
    .await?
    .ok_or(ApiError::NotFound)
}

pub async fn update_kill(state: &ApiContext, kill_id: Uuid, kill_event_record: KillEventRecord) -> Result<KillEventRecord, ApiError> {
    let record: KillEventRecord = sqlx::query_as(r#"
        update kill_event 
        set 
            killer_id = coalesce($2, killer_id),
            victim_id = coalesce($3, killer_id),
            status = coalesce($4, status),
            evidence_resource_id = coalesce($5, evidence_resource_id),
            moderation_reason = coalesce($6, moderation_reason),
            details = coalesce($7, details),
            killer_confirmed_at = coalesce($8, killer_confirmed_at),
            victim_confirmed_at = coalesce($9, victim_confirmed_at),
            confirmed_at = coalesce($10, confirmed_at),
            moderated_at = coalesce($11, moderated_at),
            moderator_id = coalesce($12, moderator_id),
            moderation_reason = coalesce($13, moderation_reason),
            rating_applied_at = coalesce($10, rating_applied_at)
         where kill_event_id = $1
    "#)
        .bind(db_uuid(kill_id))
        .bind(db_uuid(kill_event_record.killer_id))
        .bind(db_uuid(kill_event_record.victim_id))
        .bind(kill_event_record.status)
        .bind(db_optional_uuid(kill_event_record.evidence_resource_id))
        .bind(db_json(&kill_event_record.details))
        .bind(db_optional_timestamp(kill_event_record.killer_confirmed_at))
        .bind(db_optional_timestamp(kill_event_record.victim_confirmed_at))
        .bind(db_optional_timestamp(kill_event_record.confirmed_at))
        .bind(db_optional_timestamp(kill_event_record.moderated_at))
        .bind(db_optional_uuid(kill_event_record.moderator_id))
        .bind(kill_event_record.moderation_reason)
        .bind(db_optional_timestamp(kill_event_record.rating_applied_at))
        .fetch_one(&state.db)
        .await?;
    
    todo!()
}
