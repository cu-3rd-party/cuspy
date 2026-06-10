use axum::extract::State;
use http::HeaderMap;
use axum::Json;
use crate::api::helpers;
use crate::api::models::{db_uuid, ApiError};
use crate::api::models::profile::{ProfileRequestRecord, ProfileRequestResponse};
use crate::ApiContext;

pub async fn list_profile_requests(
    State(state): State<ApiContext>,
    headers: HeaderMap,
) -> Result<Json<Vec<ProfileRequestResponse>>, ApiError> {
    helpers::optional_telegram_user_id(&headers, &state)?;
    let auth = helpers::require_bearer_token(&headers, &state)?;
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
    .bind(db_uuid(auth.user_id))
    .fetch_all(&state.db)
    .await?;

    Ok(Json(
        requests
            .into_iter()
            .map(helpers::to_profile_request_response)
            .collect(),
    ))
}