use crate::rest::extractor::AdminUser;
use crate::rest::models::kill::KillEventResponse;
use crate::rest::models::{ApiError, db_uuid, kill};
use crate::rest::routes::kill::helpers::KILL_EVENT_COLUMNS;
use crate::{ApiContext, notifier};
use axum::Json;
use axum::extract::{Path, State};
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Clone, Copy, Deserialize, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ModerationActions {
    Approve,
    Reject,
    // а какие еще нужны то
}

#[derive(Deserialize, ToSchema)]
pub struct ModerateKillRequest {
    pub action: ModerationActions,
    pub reason: Option<String>,
}

#[utoipa::path(
    post,
    path = "/kill/{kill_id}/moderate",
    tag = "kill",
    params(("kill_id" = Uuid, Path, description = "Kill event id")),
    request_body = ModerateKillRequest,
    responses(
        (status = 200, description = "Kill event moderated", body = KillEventResponse),
        (status = 400, description = "Bad request", body = crate::rest::models::ErrorResponse),
        (status = 404, description = "Kill event not found", body = crate::rest::models::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::rest::models::ErrorResponse),
    ),
    security(("bearer_auth" = []))
)]
pub async fn moderate_kill(
    State(state): State<ApiContext>,
    AdminUser(user): AdminUser,
    Path(kill_id): Path<Uuid>,
    Json(request): Json<ModerateKillRequest>,
) -> Result<Json<KillEventResponse>, ApiError> {
    let reason = request
        .reason
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty());

    let select_query = format!(
        r#"
        select
            {KILL_EVENT_COLUMNS}
        from kill_event
        where kill_event_id = cast($1 as uuid)
        "#
    );

    let mut tx = state.db.begin().await?;
    let record = sqlx::query_as::<_, crate::rest::models::kill::KillEventRecord>(&select_query)
        .bind(db_uuid(kill_id))
        .fetch_optional(&mut *tx)
        .await?
        .ok_or(ApiError::NotFound)?;

    let should_apply_rating =
        matches!(request.action, ModerationActions::Approve) && record.rating_applied_at.is_none();

    let update_query = match request.action {
        ModerationActions::Approve => {
            if record.status != "CONFIRMED" {
                return Err(ApiError::BadRequest(
                    "kill must be confirmed before approval".into(),
                ));
            }

            Some(format!(
                r#"
                update kill_event
                set
                    status = 'ADMIN_APPROVED',
                    moderated_at = now(),
                    moderator_id = cast($2 as uuid),
                    moderation_reason = $3,
                    rating_applied_at = coalesce(rating_applied_at, now())
                where kill_event_id = cast($1 as uuid)
                returning
                    {KILL_EVENT_COLUMNS}
                "#
            ))
        }
        ModerationActions::Reject => {
            if !matches!(record.status.as_str(), "REPORTED" | "CONFIRMED") {
                return Err(ApiError::BadRequest(
                    "kill can no longer be rejected".into(),
                ));
            }

            Some(format!(
                r#"
                update kill_event
                set
                    status = 'REJECTED',
                    moderated_at = now(),
                    moderator_id = cast($2 as uuid),
                    moderation_reason = $3
                where kill_event_id = cast($1 as uuid)
                returning
                    {KILL_EVENT_COLUMNS}
                "#
            ))
        }
    };

    let record = sqlx::query_as::<_, crate::rest::models::kill::KillEventRecord>(
        update_query.as_deref().expect("query exists"),
    )
    .bind(db_uuid(kill_id))
    .bind(db_uuid(user.user_id))
    .bind(reason)
    .fetch_one(&mut *tx)
    .await?;

    if should_apply_rating {
        sqlx::query(
            r#"
            insert into rating_history (rating_history_id, user_id, rating, change, reason)
            select cast($1 as uuid), user_id, rating + $3, $3, $4
            from "user"
            where user_id = cast($2 as uuid)
            "#,
        )
        .bind(db_uuid(Uuid::now_v7()))
        .bind(db_uuid(record.killer_id))
        .bind(25_i64)
        .bind(format!("kill_approved:{}", record.kill_event_id))
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;

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
