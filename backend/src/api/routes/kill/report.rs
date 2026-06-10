use crate::api::helpers;
use crate::api::models::kill::{KillEventRecord, KillEventResponse, ReportKillRequest};
use crate::api::models::{ApiError, db_json, db_uuid, kill};
use crate::{ApiContext, notifier};
use axum::Json;
use axum::extract::State;
use http::HeaderMap;
use serde_json::{Value, json};
use uuid::Uuid;

// TODO: я правильно понимаю, что эта штука работает только для убийцы?
pub async fn report_kill(
    State(state): State<ApiContext>,
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

    let record = sqlx::query_as::<_, KillEventRecord>(
        r#"
        insert into kill_event (
            kill_event_id,
            killer_id,
            victim_id,
            details,
            killer_confirmed_at
        )
        values ($1, $2, $3, now())
        returning
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
        "#,
    )
    .bind(db_uuid(Uuid::now_v7()))
    .bind(db_uuid(auth.user_id))
    .bind(db_uuid(payload.victim_id))
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
        format!(
            "Kill report {} created and awaiting victim response.",
            record.kill_event_id
        ),
    )
    .await;

    Ok((
        http::StatusCode::CREATED,
        Json(kill::to_kill_response(record)),
    ))
}
