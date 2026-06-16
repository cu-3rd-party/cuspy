use crate::ApiContext;
use crate::models::ApiError;
use crate::models::auth::{AuthTokenPair, AuthUserRecord, EmailRegisterRequest};
use crate::notifier::notify_admins;
use crate::rest::extractor::MaybeAuthUser;
use crate::rest::helpers;
use axum::Json;
use axum::extract::State;
use http::StatusCode;

#[utoipa::path(
    post,
    path = "/api/auth/register",
    tag = "auth",
    request_body = EmailRegisterRequest,
    responses(
        (status = 201, description = "Registration succeeded", body = AuthTokenPair),
        (status = 400, description = "Bad request", body = crate::models::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::models::ErrorResponse),
    ),
    security(("bearer_auth" = []))
)]
pub async fn register(
    State(state): State<ApiContext>,
    MaybeAuthUser(user): MaybeAuthUser,
    Json(payload): Json<EmailRegisterRequest>,
) -> Result<(StatusCode, Json<AuthTokenPair>), ApiError> {
    let mut tx = state.db.begin().await?; // may be useful in future

    // let user = User::create(&mut *tx, payload.username, user.is_some_and(|u| u.is_admin), None).await?;
    // todo: here i removed rating addition. this is done by default by trigger in the db i suppose. check this

    let auth_user =
        AuthUserRecord::new_email_user(&mut *tx, None, payload.email, payload.password).await?;

    tx.commit().await?;

    if let Some(email) = auth_user.email.clone() {
        notify_admins(&state, format!("new email user: {}", email)).await;
    }

    let token_pair = helpers::create_token_pair(&state, &auth_user, user)?;
    Ok((StatusCode::CREATED, Json(token_pair)))
}
