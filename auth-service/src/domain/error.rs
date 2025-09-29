use color_eyre::eyre::Report;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthAPIError {
    #[error("User already exists")]
    UserAlreadyExists,
    #[error("User not found")]
    UserNotFound,
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Incorrect credentials")]
    IncorrectCredentials,
    #[error("Missing token")]
    MissingToken,
    #[error("Invalid token")]
    InvalidToken,
    #[error("Token already banned")]
    TokenAlreadyBanned,
    #[error("Failed to ban token")]
    TokenBanFailed,
    #[error("Unexpected error: {0}")]
    UnexpectedError(#[source] Report),
}

// TokenExpired,
// TwoFactorRequired,
// TwoFactorInvalid,
// InternalError,
