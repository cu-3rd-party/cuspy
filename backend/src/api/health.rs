use axum::extract::State;
use crate::api::models::{ApiError, HealthResponse};
use axum::Json;
use crate::AppState;

pub async fn health(
    State(state): State<AppState>,
) -> Result<Json<HealthResponse>, ApiError> {
    if !sqlx::query(r#"select 1"#).fetch_one(&state.db).await.is_ok() {
        return Err(ApiError::Internal("failed to ping db".to_string()));
    }
    Ok(Json(HealthResponse { status: "ok" }))
}
