use axum::extract::{Path, State};
use axum::Json;
use uuid::Uuid;
use crate::api::models::{db_uuid, ApiError};
use crate::api::models::resource::Resource;
use crate::ApiContext;

// используется по сути только для получения метаданных
#[utoipa::path(
    get,
    path = "/resource/{resource_id}",
    tag = "resource",
    params(("resource_id" = Uuid, Path, description = "Resource id")),
    responses(
        (status = 200, description = "Resource metadata", body = Resource),
        (status = 404, description = "Resource not found", body = crate::api::models::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::api::models::ErrorResponse),
    )
)]
pub async fn get_resource(
    State(state): State<ApiContext>,
    Path(resource_id): Path<Uuid>
) -> Result<Json<Resource>, ApiError>{
    let resource = sqlx::query_as(r#"
            select * from "resource" where resource_id = $1
        "#)
        .bind(db_uuid(resource_id))
        .fetch_one(&state.db)
        .await?;
    Ok(Json(resource))
}
