use crate::AppState;
use crate::api::models::HealthResponse;
use axum::Json;
use axum::extract::State;
use http::HeaderMap;

pub async fn create_image(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}
