use axum::{
    extract::State, http::StatusCode, Json
};
use serde::Deserialize;

use crate::domain::{AppState, AuthAPIError};

pub async fn verify_token(
    State(_state): State<AppState>,
    Json(request): Json<VerifyTokenRequest>,
) -> Result<StatusCode, AuthAPIError> {
    // Here you would typically validate the token.
    // For demonstration purposes, we'll just check if it's non-empty.
    if request.token.is_empty() {
        return Err(AuthAPIError::InvalidToken);
    }
    // If the token is valid, return a 200 OK status.
    Ok(StatusCode::OK)

}

#[derive(Debug, Deserialize)]
pub struct VerifyTokenRequest {
    pub token: String,
}
