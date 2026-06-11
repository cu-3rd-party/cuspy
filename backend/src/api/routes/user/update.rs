use crate::ApiContext;
use crate::api::extractor::AuthUser;
use crate::api::models::user::{UpdateUserRequest, UserRecord, UserResponse};
use crate::api::models::{ApiError, db_uuid};
use crate::api::{extractor, helpers};
use axum::Json;
use axum::extract::{Path, State};
use http::HeaderMap;
use uuid::Uuid;

pub async fn update_user(
    State(state): State<ApiContext>,
    AuthUser(user): AuthUser,
    Path(user_id): Path<Uuid>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>, ApiError> {
    helpers::ensure_owner(&user, user_id)?;

    let user = sqlx::query_as::<_, UserRecord>(
        r#"
        update "user"
        set
            telegram_id = coalesce($2, telegram_id),
            agent_name = coalesce($3, agent_name)
        where user_id = cast($1 as uuid)
        returning
            user_id,
            telegram_id,
            agent_name,
            is_admin,
            created_at,
            updated_at
        "#,
    )
    .bind(db_uuid(user_id))
    .bind(payload.telegram_id)
    .bind(payload.agent_name)
    .fetch_optional(&state.db)
    .await?
    .ok_or(ApiError::NotFound)?;

    Ok(Json(helpers::to_user_response(user)))
}
