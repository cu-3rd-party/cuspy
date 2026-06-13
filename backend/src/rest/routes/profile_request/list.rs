use crate::ApiContext;
use crate::models::profile::{ProfileRequestRecord, ProfileRequestResponse};
use crate::models::{ApiError, db_uuid};
use crate::rest::extractor::AuthUser;
use crate::rest::helpers;
use axum::Json;
use axum::extract::State;

#[utoipa::path(
    get,
    path = "/profile-requests",
    tag = "profile-request",
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
) -> Result<Json<Vec<ProfileRequestResponse>>, ApiError> {
    let requests = sqlx::query_as::<_, ProfileRequestRecord>(
        r#"
        select
            cast(profile_request_id as text) as profile_request_id,
            cast(user_id as text) as user_id,
            cast(requested_profile_data_id as text) as requested_profile_data_id,
            status,
            reviewer_note,
            cast(reviewed_at as text) as reviewed_at,
            cast(created_at as text) as created_at,
            cast(updated_at as text) as updated_at
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
