#[derive(Debug)]
pub enum AuthAPIError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    IncorrectCredentials,
    UnexpectedError,
    MissingToken,
    InvalidToken,
    // TokenExpired,
    // TwoFactorRequired,
    // TwoFactorInvalid,
    // InternalError,
}
