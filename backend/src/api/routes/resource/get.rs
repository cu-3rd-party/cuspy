use crate::ApiContext;
use crate::api::models::resource::Resource;
use crate::api::models::{ApiError, db_uuid};
use axum::Json;
use axum::extract::{Path, State};
use uuid::Uuid;

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
    Path(resource_id): Path<Uuid>,
) -> Result<Json<Resource>, ApiError> {
    let mut resource: Resource = sqlx::query_as(
        r#"
            select
                cast(resource_id as text) as resource_id,
                file_location,
                file_size,
                mime_type,
                checksum,
                cast(created_at as text) as created_at,
                cast(updated_at as text) as updated_at
            from "resource"
            where resource_id = cast($1 as uuid)
        "#,
    )
    .bind(db_uuid(resource_id))
    .fetch_one(&state.db)
    .await?;
    resource.file_location = state.bucket.presign_get(&resource.file_location, 1*60, None).await?;
    Ok(Json(resource))
}
