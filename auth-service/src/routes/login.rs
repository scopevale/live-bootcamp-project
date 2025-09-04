use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::domain::{AppState, User, AuthAPIError, Email, Password};

pub async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let email =
        Email::parse(request.email.clone()).map_err(|_| AuthAPIError::InvalidCredentials)?;
    let password = Password::parse(request.password.clone()).map_err(|_| AuthAPIError::InvalidCredentials)?;

    let user_store = state.user_store.read().await;
    let user: User = user_store.get_user(&email).await.map_err(|_| AuthAPIError::InvalidCredentials)?;

    dbg!("User found: {:?}", &user);

    if !user.verify_password(&password) {
        return Err(AuthAPIError::InvalidCredentials);
    }

    Ok((StatusCode::OK, Json(LoginResponse {
        message: format!("User {} logged in successfully", email),
    })))
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
