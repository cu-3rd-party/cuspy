use crate::ApiContext;
use crate::models::agent_data::AgentData;
use crate::models::{ApiError, db_uuid};
use crate::rest::extractor::AuthUser;
use axum::Json;
use axum::extract::{Path, State};
use uuid::Uuid;

#[utoipa::path(
    get,
    path = "/agent-data/{agent_data_id}",
    tag = "agent-data",
    params(("agent_data_id" = Uuid, Path, description = "Profile request id")),
    responses(
        (status = 200, description = "Agent data", body = AgentData),
        (status = 403, description = "Forbidden", body = crate::models::ErrorResponse),
        (status = 404, description = "Agent data not found", body = crate::models::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::models::ErrorResponse),
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_agent_data(
    State(state): State<ApiContext>,
    AuthUser(_user): AuthUser, // пока что не используется, но позволяет залочить за 403 эндпоинт
    Path(agent_data_id): Path<Uuid>,
) -> Result<Json<AgentData>, ApiError> {
    // пока что сделаем так что человек всегда может получить данные по айди
    // в дальнейшем может быть залочить эту логику только когда человеку реально
    // разрешено просматривать эти данные и сделать IP-TGID based lockdown
    let data = sqlx::query_as(
        r#"
            select
                cast(agent_data_id as text) as agent_data_id,
                codename,
                academic_group,
                academic_level,
                course_number,
                bachelor_track,
                identification_name,
                cast(identification_image_id as text) as identification_image_id,
                physical_contact_allowed,
                hugs_close_proximity_allowed
            from agent_data where agent_data_id = cast($1 as uuid)
        "#,
    )
    .bind(db_uuid(agent_data_id))
    .fetch_one(&state.db)
    .await?;
    Ok(Json(data))
}
