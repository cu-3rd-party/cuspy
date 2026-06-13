use crate::ApiContext;
use crate::rest::models::{ApiError, HealthResponse};
use axum::Json;
use axum::extract::State;

#[utoipa::path(
    get,
    path = "/health",
    tag = "system",
    responses(
        (status = 200, description = "Health check passed", body = HealthResponse),
        (status = 500, description = "Health check failed", body = crate::rest::models::ErrorResponse),
    )
)]
pub async fn health(State(state): State<ApiContext>) -> Result<Json<HealthResponse>, ApiError> {
    if !sqlx::query(r#"select 1"#)
        .fetch_one(&state.db)
        .await
        .is_ok()
    {
        return Err(ApiError::Internal("failed to ping db".to_string()));
    }
    Ok(Json(HealthResponse { status: "ok" }))
}
