use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::domain::{AppState, User};

pub async fn signup(
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>,
) -> Result<impl IntoResponse, ()> {
    // create a new 'User' from the request data
    let _user = User::new(
      request.email.clone(),
      request.password.clone(),
      request.requires_2fa);

    let mut _user_store = state.user_store.write().await;

    // if user_store.get_user(&user.email).await.is_ok() {
        // todo!();
        // return Err(AuthAPIError::UserAlreadyExists);
    // }

    // if user_store.add_user(user).await.is_err() {
        // todo!();
      // return Err(AuthAPIError::UnExpectedError);
    // }

    let response = Json(SignupResponse {
      message: format!("User {} created successfully", request.email),
    });

    Ok((StatusCode::CREATED, response))
}

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SignupResponse {
    pub message: String,
}
