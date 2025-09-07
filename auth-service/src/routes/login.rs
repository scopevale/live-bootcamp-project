use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};
use crate::{domain::{AppState, AuthAPIError, Email, Password, User, UserStoreError}, utils::auth::generate_auth_cookie};

pub async fn login(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(request): Json<LoginRequest>,
) -> Result<(CookieJar, impl IntoResponse), AuthAPIError> {
    let email = Email::parse(request.email.clone()).map_err(|_| AuthAPIError::InvalidCredentials)?;
    let password = Password::parse(request.password.clone()).map_err(|_| AuthAPIError::InvalidCredentials)?;

    let user_store = &state.user_store.read().await;

    user_store
        .validate_user(&email, &password)
        .await
        .map_err(|e| match e {
            UserStoreError::UserNotFound        => AuthAPIError::UserNotFound,
            UserStoreError::IncorrectCredentials => AuthAPIError::IncorrectCredentials,
            _                                    => AuthAPIError::UnexpectedError,
        })?; // early-returns Err(AuthAPIError::...)

    let user: User = user_store.get_user(&email).await.map_err(|_| AuthAPIError::IncorrectCredentials)?;

    if !user.verify_password(&password) {
        return Err(AuthAPIError::IncorrectCredentials);
    }

    let auth_cookie = match generate_auth_cookie(&user.email) {
        Ok(cookie) => cookie,
        Err(_) => return Err(AuthAPIError::UnexpectedError),
    };

    let updated_jar = jar.add(auth_cookie);

    Ok((updated_jar, StatusCode::OK.into_response()))

    // if !user.verify_password(&password) {
    //     return (jar, Err(AuthAPIError::IncorrectCredentials));
    // }
    //
    // user_store
    //     .validate_user(&email, &password)
    //     .await
    //     .map_err(|e| match e {
    //         UserStoreError::UserNotFound        => AuthAPIError::UserNotFound,
    //         UserStoreError::IncorrectCredentials => AuthAPIError::IncorrectCredentials,
    //         _                                    => AuthAPIError::UnexpectedError,
    //     })?; // early-returns Err(AuthAPIError::...)
    //
    // dbg!("User validated successfully");
    //
    // let auth_cookie = generate_auth_cookie(&email);
    //
    // let updated_jar = jar.add(&auth_cookie);
    //
    // (updated_jar, (StatusCode::OK, Json(LoginResponse {
    //     message: format!("User {} logged in successfully", email),
    // })))
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct LoginResponse {
    pub message: String,
}
