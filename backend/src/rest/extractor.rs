use crate::ApiContext;
use crate::models::ApiError;
use crate::models::user::User;
use http::request::Parts;
pub struct AuthUser(pub User);
pub struct MaybeAuthUser(pub Option<User>);
pub struct AdminUser(pub User);

impl axum::extract::FromRequestParts<ApiContext> for AuthUser {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &ApiContext,
    ) -> Result<Self, Self::Rejection> {
        Ok(Self(User::from_headers(&state, &parts.headers)?))
    }
}

impl axum::extract::FromRequestParts<ApiContext> for MaybeAuthUser {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &ApiContext,
    ) -> Result<Self, Self::Rejection> {
        Ok(Self(User::from_headers(&state, &parts.headers).ok()))
    }
}

impl axum::extract::FromRequestParts<ApiContext> for AdminUser {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &ApiContext,
    ) -> Result<Self, Self::Rejection> {
        let user = User::from_headers(&state, &parts.headers)?;

        if !user.is_admin {
            return Err(ApiError::Unauthorized);
        }
        Ok(Self(user))
    }
}
