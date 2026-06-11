use crate::ApiContext;
use crate::api::models::profile::{ProfileRequestRecord, ProfileRequestResponse};
use crate::api::models::{db_uuid, ApiError};
use crate::api::{extractor, helpers};
use axum::Json;
use axum::extract::{Path, State};
use http::HeaderMap;
use uuid::Uuid;
use crate::api::extractor::AuthUser;

pub async fn get_profile_request(
    State(state): State<ApiContext>,
    AuthUser(user): AuthUser,
    Path(request_id): Path<Uuid>,
) -> Result<Json<ProfileRequestResponse>, ApiError> {
    let request = sqlx::query_as::<_, ProfileRequestRecord>(
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
