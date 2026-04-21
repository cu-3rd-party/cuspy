use axum::Json;
use crate::api::models::HealthResponse;

pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}