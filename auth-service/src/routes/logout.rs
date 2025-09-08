use axum::{
    http::StatusCode,
    response::IntoResponse
};
use axum_extra::extract::{cookie, CookieJar};

use crate::{domain::AuthAPIError, utils::{auth::validate_token, constants::JWT_COOKIE_NAME}};

pub async fn logout(
    // State(state): State<AppState>,
    jar: CookieJar
) -> Result<(CookieJar, impl IntoResponse), AuthAPIError> {
    // Retrieve the JWT cookie from the CookieJar
    // Return AuthAPIError::MissingToken if the cookie is not found
    let cookie = match jar.get(JWT_COOKIE_NAME) {
        Some(cookie) => cookie,
        None => return Err(AuthAPIError::MissingToken),
    };

    let token = cookie.value().to_owned();

    let _ = match validate_token(&token).await {
        Ok(_) => (),
        Err(_) => return Err(AuthAPIError::InvalidToken),
    };

    // Add the token to the blacklist
    // TODO: Implement token blacklist logic here

    // Remove the JWT cookie from the CookieJar
    let jar = jar.remove(cookie::Cookie::from(JWT_COOKIE_NAME));

    Ok((jar, StatusCode::OK.into_response()))
}

