use crate::models::kill::{KillEventRecord, KillEventResponse, ReportKillRequest};
use crate::models::{ApiError, db_json, db_uuid, kill};
use crate::rest::extractor::AuthUser;
use crate::rest::routes::kill::helpers::KILL_EVENT_COLUMNS;
use crate::{ApiContext, notifier};
use axum::Json;
use axum::extract::State;
use serde_json::{Value, json};
use uuid::Uuid;

#[utoipa::path(
    post,
    path = "/api/kill",
    tag = "kill",
    request_body = ReportKillRequest,
    responses(
        (status = 201, description = "Kill event reported", body = KillEventResponse),
        (status = 400, description = "Bad request", body = crate::models::ErrorResponse),
        (status = 403, description = "Forbidden", body = crate::models::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::models::ErrorResponse),
    ),
    security(("bearer_auth" = []))
)]
pub async fn report_kill(
    State(state): State<ApiContext>,
    AuthUser(user): AuthUser,
    Json(payload): Json<ReportKillRequest>,
) -> Result<(http::StatusCode, Json<KillEventResponse>), ApiError> {
    let (killer_id, victim_id) = match (payload.killer_id, payload.victim_id) {
        (Some(killer_id), Some(victim_id)) => (killer_id, victim_id),
        (None, Some(victim_id)) => (user.user_id, victim_id),
        (Some(killer_id), None) => (killer_id, user.user_id),
        (None, None) => {
            return Err(ApiError::BadRequest(
                "either victim_id or killer_id must be provided".into(),
            ));
        }
    };

    if killer_id == victim_id {
        return Err(ApiError::BadRequest("killer and victim must differ".into()));
    }

    if user.user_id != killer_id && user.user_id != victim_id {
        return Err(ApiError::Forbidden);
    }

    let mut details = match payload.details {
        Some(Value::Object(map)) => Value::Object(map),
        Some(_) => {
            return Err(ApiError::BadRequest(
                "kill details must be a JSON object".into(),
            ));
        }
        None => json!({}),
    };

    if let Some(evidence_url) = payload.evidence_url
        && let Some(details_map) = details.as_object_mut()
    {
        details_map.insert("evidence_url".into(), Value::String(evidence_url));
    }

    let reporter_is_killer = user.user_id == killer_id;
    let query = format!(
        r#"
        insert into kill_event (
            kill_event_id,
            killer_id,
            victim_id,
            status,
            evidence_resource_id,
            details,
            killer_confirmed_at,
            victim_confirmed_at
        )
        values (
            cast($1 as uuid),
            cast($2 as uuid),
            cast($3 as uuid),
            'REPORTED',
            null,
            cast($4 as jsonb),
            case when $5 then now() else null end,
            case when $5 then null else now() end
        )
        returning
            {KILL_EVENT_COLUMNS}
        "#
    );

    let record = sqlx::query_as::<_, KillEventRecord>(&query)
        .bind(db_uuid(Uuid::now_v7()))
        .bind(db_uuid(killer_id))
        .bind(db_uuid(victim_id))
        .bind(db_json(&details))
        .bind(reporter_is_killer)
        .fetch_one(&state.db)
        .await?;

    let counterparty_id = if reporter_is_killer {
        victim_id
    } else {
        killer_id
    };

    notifier::notify_user(
        &state,
        counterparty_id,
        format!(
            "Kill report {} filed. Open the confirmation flow to verify the event.",
            record.kill_event_id
        ),
    )
    .await;
    notifier::notify_admins(
        &state,
        format!(
            "Kill report {} created and awaiting counterparty confirmation.",
            record.kill_event_id
        ),
    )
    .await;

    Ok((
        http::StatusCode::CREATED,
        Json(kill::to_kill_response(record)),
    ))
}
