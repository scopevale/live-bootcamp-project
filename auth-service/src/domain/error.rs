// use crate::domain::UserStoreError;

// #[derive(Debug)]
pub enum AuthAPIError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    IncorrectCredentials,
    UnexpectedError,
    // InvalidToken,
    // TokenExpired,
    // TwoFactorRequired,
    // TwoFactorInvalid,
    // InternalError,
}

// impl From<UserStoreError> for AuthAPIError {
//     fn from(error: UserStoreError) -> Self {
//         match error {
//             UserStoreError::UserAlreadyExists        => AuthAPIError::UserAlreadyExists,
//             UserStoreError::UserNotFound             => AuthAPIError::UserNotFound,
//             UserStoreError::InvalidCredentials       => AuthAPIError::InvalidCredentials,
//             UserStoreError::IncorrectCredentials     => AuthAPIError::IncorrectCredentials,
//             UserStoreError::UnexpectedError          => AuthAPIError::UnexpectedError,
//         }
//     }
// }
