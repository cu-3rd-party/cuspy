use crate::ApiContext;
use crate::rest::extractor::AuthUser;
use crate::rest::helpers;
use crate::rest::models::profile::{ProfileRequestRecord, ProfileRequestResponse};
use crate::rest::models::{ApiError, db_uuid};
use axum::Json;
use axum::extract::{Path, State};
use uuid::Uuid;

#[utoipa::path(
    get,
    path = "/profile-requests/{request_id}",
    tag = "profile-request",
    params(("request_id" = Uuid, Path, description = "Profile request id")),
    responses(
        (status = 200, description = "Profile request", body = ProfileRequestResponse),
        (status = 403, description = "Forbidden", body = crate::rest::models::ErrorResponse),
        (status = 404, description = "Profile request not found", body = crate::rest::models::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::rest::models::ErrorResponse),
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_profile_request(
    State(state): State<ApiContext>,
    AuthUser(user): AuthUser,
    Path(request_id): Path<Uuid>,
) -> Result<Json<ProfileRequestResponse>, ApiError> {
    let request = sqlx::query_as::<_, ProfileRequestRecord>(
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
        where profile_request_id = cast($1 as uuid)
        "#,
    )
    .bind(db_uuid(request_id))
    .fetch_optional(&state.db)
    .await?
    .ok_or(ApiError::NotFound)?;

    helpers::ensure_owner(&user, request.user_id)?;
    Ok(Json(helpers::to_profile_request_response(request)))
}
