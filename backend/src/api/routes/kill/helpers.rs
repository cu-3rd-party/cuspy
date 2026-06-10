use crate::ApiContext;
use crate::api::models::kill::KillEventRecord;
use crate::api::models::{ApiError, db_uuid};
use uuid::Uuid;

pub async fn fetch_kill(state: &ApiContext, kill_id: Uuid) -> Result<KillEventRecord, ApiError> {
    sqlx::query_as::<_, KillEventRecord>(
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
    )
    .bind(db_uuid(kill_id))
    .fetch_optional(&state.db)
    .await?
    .ok_or(ApiError::NotFound)
}
