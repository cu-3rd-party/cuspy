use crate::ApiContext;
use crate::api::extractor::AuthUser;
use crate::api::models::profile::{ProfileRequestRecord, ProfileRequestResponse};
use crate::api::models::{ApiError, db_uuid};
use crate::api::{extractor, helpers};
use axum::Json;
use axum::extract::State;
use http::HeaderMap;

#[utoipa::path(
    get,
    path = "/profile-requests",
    tag = "profile-request",
    responses(
        (status = 200, description = "Current user's profile requests", body = [ProfileRequestResponse]),
        (status = 401, description = "Unauthorized", body = crate::api::models::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::api::models::ErrorResponse),
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_profile_requests(
    State(state): State<ApiContext>,
    AuthUser(user): AuthUser,
) -> Result<Json<Vec<ProfileRequestResponse>>, ApiError> {
    let requests = sqlx::query_as::<_, ProfileRequestRecord>(
        r#"
        select
            profile_request_id,
            user_id,
            requested_profile_data_id,
            status,
            reviewer_note,
            reviewed_at,
            created_at,
            updated_at
        from profile_request
        where cast(user_id as text) = $1
        order by created_at desc
        "#,
    )
    .bind(db_uuid(user.user_id))
    .fetch_all(&state.db)
    .await?;

    Ok(Json(
        requests
            .into_iter()
            .map(helpers::to_profile_request_response)
            .collect(),
    ))
}
