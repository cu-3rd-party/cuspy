use crate::ApiContext;
use crate::models::ApiError;
use crate::models::kill::{ConfirmKillRequest, KillEventResponse};
use crate::rest::extractor::AuthUser;
use axum::Json;
use axum::extract::{Path, State};
use uuid::Uuid;

#[utoipa::path(
    post,
    path = "/api/kill/{kill_id}/confirm",
    tag = "kill",
    params(("kill_id" = Uuid, Path, description = "Kill event id")),
    request_body = ConfirmKillRequest,
    responses(
        (status = 200, description = "Kill event confirmation updated", body = KillEventResponse),
        (status = 400, description = "Bad request", body = crate::models::ErrorResponse),
        (status = 403, description = "Forbidden", body = crate::models::ErrorResponse),
        (status = 404, description = "Kill event not found", body = crate::models::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::models::ErrorResponse),
    ),
    security(("bearer_auth" = []))
)]
pub async fn confirm_kill(
    State(_state): State<ApiContext>,
    AuthUser(_user): AuthUser,
    Path(_kill_id): Path<Uuid>,
    Json(_req): Json<ConfirmKillRequest>,
) -> Result<Json<KillEventResponse>, ApiError> {
    todo!()
}
