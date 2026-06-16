use axum::extract::State;
use axum::Json;
use jsonwebtoken::{decode, DecodingKey, Validation};
use crate::ApiContext;
use crate::models::ApiError;
use crate::models::auth::{AuthTokenPair, AuthUserRecord, RefreshClaims, RefreshTokenRequest};
use crate::models::user::User;
use crate::rest::extractor::MaybeAuthUser;
use crate::rest::helpers;


#[utoipa::path(
    post,
    path = "/api/auth/refresh",
    tag = "auth",
    request_body = RefreshTokenRequest,
    responses(
        (status = 200, description = "Login succeeded", body = AuthTokenPair),
        (status = 400, description = "Bad request", body = crate::models::ErrorResponse),
        (status = 401, description = "Unauthorized", body = crate::models::ErrorResponse),
        (status = 403, description = "Forbidden", body = crate::models::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::models::ErrorResponse),
    ),
)]
pub async fn refresh_token_pair(
    State(state): State<ApiContext>,
    MaybeAuthUser(user): MaybeAuthUser,
    Json(req): Json<RefreshTokenRequest>,
) -> Result<Json<AuthTokenPair>, ApiError> {
    let mut tx = state.db.begin().await?;
    if let Some(user) = user {
        // к нам пришел чел с валидным аксес токеном
        let auth_user_records = AuthUserRecord::get_by_user_id(&mut *tx, user.user_id).await.ok_or(ApiError::NotFound)?;
        tx.commit().await?;
        return Ok(Json(helpers::create_token_pair(&state, auth_user_records.get(0).ok_or(ApiError::NotFound)?, Some(user))?));
    }
    let decoded = decode::<RefreshClaims>(
        &req.refresh_token,
        &DecodingKey::from_secret(state.jwt_secret.as_bytes()),
        &Validation::default(),
    )
        .map_err(|_| ApiError::Unauthorized)?
        .claims;
    let auth_user_record = AuthUserRecord::get_by_id(&mut *tx, decoded.auth_user_id).await.ok_or(ApiError::NotFound)?;
    let user = User::get_by_option_id(&mut *tx, auth_user_record.user_id).await;
    let tokens = helpers::create_token_pair(&state, &auth_user_record, user)?;
    tx.commit().await?;

    Ok(Json(tokens))
}