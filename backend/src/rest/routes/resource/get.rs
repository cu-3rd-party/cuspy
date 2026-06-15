use crate::ApiContext;
use crate::models::resource::Resource;
use crate::models::ApiError;
use axum::Json;
use axum::extract::{Path, State};
use uuid::Uuid;

// используется по сути только для получения метаданных
#[utoipa::path(
    get,
    path = "/api/resource/{resource_id}",
    tag = "resource",
    params(("resource_id" = Uuid, Path, description = "Resource id")),
    responses(
        (status = 200, description = "Resource metadata", body = Resource),
        (status = 404, description = "Resource not found", body = crate::models::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::models::ErrorResponse),
    )
)]
pub async fn get_resource(
    State(state): State<ApiContext>,
    Path(resource_id): Path<Uuid>,
) -> Result<Json<Resource>, ApiError> {
    let resource = Resource::get_by_id(&state.db, resource_id)
        .await
        .ok_or(ApiError::NotFound)?;
    Ok(Json(resource))
}
