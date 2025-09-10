#[derive(Debug)]
pub enum AuthAPIError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    IncorrectCredentials,
    UnexpectedError,
    MissingToken,
    InvalidToken,
    TokenAlreadyBanned,
    TokenBanFailed,
    // TokenExpired,
    // TwoFactorRequired,
    // TwoFactorInvalid,
    // InternalError,
}
