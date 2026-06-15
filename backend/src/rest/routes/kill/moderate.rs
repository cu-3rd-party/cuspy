use crate::ApiContext;
use crate::models::ApiError;
use crate::models::kill::KillEventResponse;
use crate::rest::extractor::AdminUser;
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
    path = "/api/kill/{kill_id}/moderate",
    tag = "kill",
    params(("kill_id" = Uuid, Path, description = "Kill event id")),
    request_body = ModerateKillRequest,
    responses(
        (status = 200, description = "Kill event moderated", body = KillEventResponse),
        (status = 400, description = "Bad request", body = crate::models::ErrorResponse),
        (status = 404, description = "Kill event not found", body = crate::models::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::models::ErrorResponse),
    ),
    security(("bearer_auth" = []))
)]
pub async fn moderate_kill(
    State(_state): State<ApiContext>,
    AdminUser(_user): AdminUser,
    Path(_kill_id): Path<Uuid>,
    Json(_request): Json<ModerateKillRequest>,
) -> Result<Json<KillEventResponse>, ApiError> {
    todo!()
}
