use crate::{
    domain::{AppState, AuthAPIError, Email, Password, User},
    services::data_stores::{LoginAttemptId, TwoFACode, UserStoreError},
    utils::auth::generate_auth_cookie,
};
use axum::{extract::State, http::StatusCode, Json};
use axum_extra::extract::CookieJar;
use color_eyre::eyre::Report;
use serde::{Deserialize, Serialize};
use tracing::instrument;

#[axum_macros::debug_handler]
#[instrument(name = "Login", skip_all)]
pub async fn login(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(request): Json<LoginRequest>,
) -> Result<(CookieJar, (StatusCode, Json<LoginResponse>)), AuthAPIError> {
    let email =
        Email::parse(request.email.clone()).map_err(|_| AuthAPIError::InvalidCredentials)?;
    let password = Password::parse(request.password.clone().into())
        .map_err(|_| AuthAPIError::InvalidCredentials)?;

    let user_store = &state.user_store.read().await;

    user_store
        .validate_user(&email, &password)
        .await
        .map_err(|e| match e {
            UserStoreError::UserNotFound => AuthAPIError::UserNotFound,
            UserStoreError::IncorrectCredentials => AuthAPIError::IncorrectCredentials,
            _ => AuthAPIError::UnexpectedError(Report::msg(format!(
                "Unexpected error during user validation: {:?}",
                e
            ))),
        })?; // early-returns Err(AuthAPIError::...)

    let user: User = user_store
        .get_user(&email)
        .await
        .map_err(|_| AuthAPIError::IncorrectCredentials)?;

    // Handle request based on user's 2FA configuration
    match user.requires_2fa {
        true => handle_2fa(&user.email, &state, jar).await,
        false => handle_no_2fa(&user.email, jar).await,
    }
}

// New!
#[instrument(name = "Handle 2fa", skip_all)]
async fn handle_2fa(
    email: &Email,
    state: &AppState,
    jar: CookieJar,
) -> Result<(CookieJar, (StatusCode, Json<LoginResponse>)), AuthAPIError> {
    // First, generate a unique login attempt ID & 2FA code
    let login_attempt_id = LoginAttemptId::default();
    let two_fa_code = TwoFACode::default();

    if let Err(e) = state
        .two_fa_code_store
        .write()
        .await
        .add_code(email.clone(), login_attempt_id.clone(), two_fa_code.clone())
        .await
    {
        return Err(AuthAPIError::UnexpectedError(e.into()));
    }

    if let Err(e) = state
        .email_client
        .send_email(email, "2FA Code", two_fa_code.as_ref())
        .await
    {
        return Err(AuthAPIError::UnexpectedError(e));
    }

    let response = Json(LoginResponse::TwoFactorAuth(TwoFactorAuthResponse {
        message: "2FA required".to_owned(),
        login_attempt_id: login_attempt_id.as_ref().to_owned(),
    }));

    Ok((jar, (StatusCode::PARTIAL_CONTENT, response)))
}

#[instrument(name = "Handle no 2fa", skip_all)]
async fn handle_no_2fa(
    email: &Email,
    jar: CookieJar,
) -> Result<(CookieJar, (StatusCode, Json<LoginResponse>)), AuthAPIError> {
    let auth_cookie = match generate_auth_cookie(email) {
        Ok(cookie) => cookie,
        Err(e) => {
            return Err(AuthAPIError::UnexpectedError(e));
        }
    };

    let updated_jar = jar.add(auth_cookie);

    Ok((
        updated_jar,
        (StatusCode::OK, Json(LoginResponse::RegularAuth)),
    ))
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

// The login route can return 2 possible success responses.
// This enum models each response!
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum LoginResponse {
    RegularAuth,
    TwoFactorAuth(TwoFactorAuthResponse),
}

// // If a user requires 2FA, this JSON body should be returned!
#[derive(Debug, Serialize, Deserialize)]
pub struct TwoFactorAuthResponse {
    pub message: String,
    #[serde(rename = "loginAttemptId")]
    pub login_attempt_id: String,
}
