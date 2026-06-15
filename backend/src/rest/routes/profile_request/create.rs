use crate::ApiContext;
use crate::models::ApiError;
use crate::models::agent_data::AgentData;
use crate::models::profile::{ProfileRequestRecord, ProfileRequestResponse};
use crate::models::resource::Resource;
use crate::rest::extractor::AuthUser;
use axum::extract::{Multipart, State};
use axum::response::Json;
use utoipa::ToSchema;

#[derive(ToSchema)]
pub struct CreateProfileRequestMultipart {
    #[schema(
        example = r#"{"codename":"Cipher","physical_contact_allowed":true,"hugs_close_proximity_allowed":false}"#
    )]
    pub data: String,
    #[schema(value_type = String, format = Binary)]
    pub image: Option<String>,
}

#[utoipa::path(
    post,
    path = "/api/profile-requests",
    tag = "profile-request",
    request_body(
        content = CreateProfileRequestMultipart,
        content_type = "multipart/form-data",
        description = "Multipart form with a `data` field containing JSON-encoded `AgentDataMetadata` and an optional `image` file"
    ),
    responses(
        (status = 200, description = "Profile data created", body = AgentData),
        (status = 400, description = "Bad request", body = crate::models::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::models::ErrorResponse),
    )
)]
pub async fn create_profile_request(
    State(state): State<ApiContext>,
    AuthUser(user): AuthUser,
    mut multipart: Multipart,
) -> Result<Json<ProfileRequestResponse>, ApiError> {
    let mut metadata: Option<crate::models::agent_data::AgentDataMetadata> = None;
    let mut image = None;
    // осознанно не использую тут транзакцию чтоб не заставлять ее висеть пока я читаю боди запроса
    if ProfileRequestRecord::get_by_user_id(&state.db, user.user_id)
        .await?
        .into_iter()
        .any(|r| r.status == "sent".to_string())
    {
        return Err(ApiError::BadRequest(
            "you already have a pending profile".to_string(),
        ));
    }

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| ApiError::BadRequest(e.to_string()))?
    {
        let name = field
            .name()
            .ok_or_else(|| ApiError::BadRequest("no field name".to_string()))?
            .to_string();

        match name.as_str() {
            "data" => {
                let text = field.text().await.map_err(|e| {
                    ApiError::BadRequest(format!("failed to parse field body: {e}"))
                })?;

                metadata = Some(serde_json::from_str(&text).map_err(|e| {
                    ApiError::BadRequest(format!("failed to parse json body: {e}"))
                })?);
            }
            "image" => {
                let content_type = field.content_type().map(String::from);
                let content = field.bytes().await.map_err(|e| {
                    ApiError::BadRequest(format!("failed to parse field body: {e}"))
                })?;
                image = Some((content, content_type));
            }
            _ => {
                return Err(ApiError::BadRequest(format!(
                    "unknown multipart field name provided: {name}"
                )));
            }
        };
    }

    let metadata = metadata.ok_or_else(|| ApiError::BadRequest("no data supplied".to_string()))?;
    let mut tx = state.db.begin().await?;
    let resource = match image {
        Some((content, content_type)) => {
            Some(Resource::new(&mut *tx, state.bucket.clone(), content, content_type).await?)
        }
        None => None,
    };
    let agent_data = AgentData::create(&mut *tx, metadata, resource).await?;
    let profile = ProfileRequestRecord::create(
        &mut *tx,
        user.user_id,
        agent_data.agent_data_id,
        "sent".to_string(),
    )
    .await?;
    let response = profile.into_response(&mut *tx).await?;
    tx.commit().await?;

    Ok(Json(response))
}
