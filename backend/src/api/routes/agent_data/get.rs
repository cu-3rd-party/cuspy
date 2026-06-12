use crate::ApiContext;
use crate::api::extractor::AuthUser;
use crate::api::models::agent_data::AgentData;
use crate::api::models::{ApiError, db_uuid};
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
        (status = 403, description = "Forbidden", body = crate::api::models::ErrorResponse),
        (status = 404, description = "Agent data not found", body = crate::api::models::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::api::models::ErrorResponse),
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
            select * from agent_data where agent_data_id = cast($1 as uuid)
        "#,
    )
    .bind(db_uuid(agent_data_id))
    .fetch_one(&state.db)
    .await?;
    Ok(Json(data))
}
