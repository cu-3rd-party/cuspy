use crate::rest::helpers::format_timestamp;
use crate::ApiContext;
use crate::models::agent_data::AgentData;
use crate::models::profile::{ProfileRequestRecord, ProfileRequestResponse};
use crate::models::ApiError;
use crate::rest::extractor::AuthUser;
use axum::Json;
use axum::extract::{Query, State};
use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Deserialize, IntoParams)]
pub struct ListParams {
    all: bool,
}

#[utoipa::path(
    get,
    path = "/api/profile-requests",
    tag = "profile-request",
    params(ListParams),
    responses(
        (status = 200, description = "Current user's profile requests", body = [ProfileRequestResponse]),
        (status = 401, description = "Unauthorized", body = crate::models::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::models::ErrorResponse),
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_profile_requests(
    State(state): State<ApiContext>,
    AuthUser(user): AuthUser,
    Query(query): Query<ListParams>,
) -> Result<Json<Vec<ProfileRequestResponse>>, ApiError> {
    let mut tx = state.db.begin().await?;
    let requests: Vec<ProfileRequestRecord>;
    if query.all && user.is_admin {
        requests = ProfileRequestRecord::get_all(&mut *tx).await?;
    } else {
        requests = ProfileRequestRecord::get_by_user_id(&mut *tx, user.user_id).await?;
    }

    let agent_data_ids: Vec<_> = requests
        .iter()
        .map(|r| r.requested_profile_data_id)
        .collect();
    let agent_data_map = AgentData::get_by_ids(&mut *tx, &agent_data_ids).await;

    let requests = requests
        .into_iter()
        .map(|r| ProfileRequestResponse {
            profile_request_id: r.profile_request_id,
            user_id: r.user_id,
            requested_profile_data: agent_data_map.get(&r.requested_profile_data_id).cloned(),
            status: r.status,
            reviewer_note: r.reviewer_note,
            reviewed_at: r.reviewed_at.map(format_timestamp),
            created_at: format_timestamp(r.created_at),
            updated_at: r.updated_at.map(format_timestamp),
        })
        .collect();

    tx.commit().await?;

    Ok(Json(requests))
}
