use axum::{
    extract::State, http::StatusCode, Json
};
use serde::Deserialize;

use crate::{domain::{AppState, AuthAPIError}, utils::auth::validate_token};

pub async fn verify_token(
    State(_state): State<AppState>,
    Json(request): Json<VerifyTokenRequest>,
) -> Result<StatusCode, AuthAPIError> {
    match validate_token(&request.token).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(AuthAPIError::InvalidToken),
    }
}

#[derive(Debug, Deserialize)]
pub struct VerifyTokenRequest {
    pub token: String,
}
