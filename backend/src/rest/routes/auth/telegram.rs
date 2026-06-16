use crate::ApiContext;
use crate::models::ApiError;
use crate::models::auth::{AuthTokenPair, AuthUserRecord, TelegramInitDataRequest};
use crate::models::user::User;
use crate::rest::extractor::MaybeAuthUser;
use crate::rest::helpers;
use crate::telegram::TelegramInitData;
use axum::Json;
use axum::extract::State;
use crate::notifier::notify_admins;

#[utoipa::path(
    post,
    path = "/api/auth/telegram",
    tag = "auth",
    request_body = TelegramInitDataRequest,
    responses(
        (status = 200, description = "Login succeeded", body = AuthTokenPair),
        (status = 400, description = "Bad request", body = crate::models::ErrorResponse),
        (status = 401, description = "Unauthorized", body = crate::models::ErrorResponse),
        (status = 403, description = "Forbidden", body = crate::models::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::models::ErrorResponse),
    ),
)]
pub async fn telegram_login_request(
    State(state): State<ApiContext>,
    MaybeAuthUser(user): MaybeAuthUser,
    Json(req): Json<TelegramInitDataRequest>,
) -> Result<Json<AuthTokenPair>, ApiError> {
    let mut tx = state.db.begin().await?;
    if let Some(user) = user {
        // к нам пришел чел с валидным аксес токеном
        let auth_users = AuthUserRecord::get_by_user_id(&mut *tx, user.user_id)
            .await
            .ok_or(ApiError::NotFound)?;
        tx.commit().await?;
        return Ok(Json(helpers::create_token_pair(
            &state,
            auth_users.get(0).ok_or(ApiError::NotFound)?,
            Some(user),
        )?));
    }
    let data = TelegramInitData::from_header(&state.telegram_bot_token, &req.init_data)
        .ok_or(ApiError::Forbidden)?;
    let existing_auth_user = AuthUserRecord::get_by_telegram_id(&mut *tx, data.user.id).await;
    if let Some(auth_user) = existing_auth_user {
        let user = User::get_by_option_id(&mut *tx, auth_user.user_id).await;
        let tokens = helpers::create_token_pair(&state, &auth_user, user)?;
        tx.commit().await?;

        return Ok(Json(tokens));
    }
    // если человек регистрируется, то у него нет уже юзера
    let auth_user = AuthUserRecord::new_telegram_user(&mut *tx, None, data.user.id).await?;
    let tokens = helpers::create_token_pair(&state, &auth_user, None)?;
    tx.commit().await?;

    if let Some(telegram_id) = auth_user.telegram_id.clone() {
        notify_admins(&state, format!("new telegram user: {}", telegram_id)).await;
    }

    Ok(Json(tokens))
}
