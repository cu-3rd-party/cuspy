use crate::ApiContext;
use crate::api::models::ApiError;
use crate::api::models::agent_data::{AgentData, AgentDataMetadata};
use axum::Json;
use axum::extract::{Multipart, State};
use http::HeaderMap;

pub async fn create_agent_data(
    State(state): State<ApiContext>,
    _headers: HeaderMap,
    mut multipart: Multipart,
) -> Result<Json<AgentData>, ApiError> {
    let mut agent_data: Option<AgentData> = None;
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| ApiError::BadRequest(e.body_text()))?
    {
        let name = field
            .name()
            .ok_or_else(|| ApiError::BadRequest("no field name".to_string()))?
            .to_string();

        match name.as_str() {
            "data" => {
                // иные метаданные
                let req: AgentDataMetadata =
                    serde_json::from_str(&field.text().await.map_err(|e| {
                        ApiError::BadRequest(format!("failed to parse field body: {e}"))
                    })?)
                    .map_err(|e| ApiError::BadRequest(format!("failed to parse json body: {e}")))?;
                agent_data = Some(sqlx::query_as(r#"
                    insert into "agent_data" (codename, academic_group, academic_level, course_number, bachelor_track, identification_name, physical_contact_allowed, hugs_close_proximity_allowed)
                    values ($1, $2, $3, $4, $5, $6, $7, $8)
                    returning
                        agent_data_id,
                        codename,
                        academic_group,
                        academic_level,
                        course_number,
                        bachelor_track,
                        identification_name,
                        identification_image_id,
                        physical_contact_allowed,
                        hugs_close_proximity_allowed
                "#)
                    .bind(req.codename)
                    .bind(req.academic_group)
                    .bind(req.academic_level.map(|s| s.to_string()))
                    .bind(req.course_number)
                    .bind(req.bachelor_track.map(|s| s.to_string()))
                    .bind(req.identification_name)
                    .bind(req.physical_contact_allowed)
                    .bind(req.hugs_close_proximity_allowed)
                    .fetch_one(&state.db)
                    .await?);
            }
            "image" => {
                // создать ресурс и сохранить айди
                // let resouce = sqlx::query_as()
            }
            _ => {
                return Err(ApiError::BadRequest(format!(
                    "unknown multipart field name provided: {name}"
                )));
            }
        };
    }

    if let None = agent_data {
        return Err(ApiError::BadRequest("no data supplied".to_string()));
    }

    Ok(Json(agent_data.expect("validated above")))
}
