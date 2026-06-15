use crate::ApiContext;
use crate::models::ApiError;
use crate::models::auth::{AuthTokenPair, AuthUserRecord, EmailLoginRequest};
use crate::models::user::User;
use crate::rest::extractor::MaybeAuthUser;
use crate::rest::helpers;
use axum::Json;
use axum::extract::State;

#[utoipa::path(
    post,
    path = "/api/auth/login",
    tag = "auth",
    request_body = EmailLoginRequest,
    responses(
        (status = 200, description = "Login succeeded", body = AuthTokenPair),
        (status = 400, description = "Bad request", body = crate::models::ErrorResponse),
        (status = 401, description = "Unauthorized", body = crate::models::ErrorResponse),
        (status = 403, description = "Forbidden", body = crate::models::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::models::ErrorResponse),
    ),
    security(("bearer_auth" = []))
)]
pub async fn login(
    State(state): State<ApiContext>,
    MaybeAuthUser(mut user): MaybeAuthUser,
    Json(payload): Json<EmailLoginRequest>,
) -> Result<Json<AuthTokenPair>, ApiError> {
    let mut tx = state.db.begin().await?;
    let auth_user = AuthUserRecord::get_by_email(&mut *tx, payload.email)
        .await
        .ok_or(ApiError::BadRequest("check email or password".to_string()))?; // obfuscation of db users emails
    // не должно быть ситуации, где у пользователя нет пароля
    if let Some(password_hash) = auth_user.password_hash.clone() {
        helpers::verify_password(&password_hash, &payload.password)?;
    } else {
        // но на всякий случай
        return Err(ApiError::Internal(
            "auth user doesn't have password hash. refusing to validate".to_string(),
        ));
    }
    if let Some(user_id) = auth_user.user_id {
        user = User::get_by_id(&mut *tx, user_id).await;
    }

    let token_pair = helpers::create_token_pair(&state, &auth_user, user)?;

    Ok(Json(token_pair))
}
