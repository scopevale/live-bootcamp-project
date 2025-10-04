use crate::domain::{Email, Password, User};
use color_eyre::eyre::{eyre, Context, Report, Result};
use secrecy::{ExposeSecret, Secret};
use thiserror::Error;

#[async_trait::async_trait]
pub trait UserStore: std::fmt::Debug {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError>;
    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError>;
    async fn validate_user(&self, email: &Email, password: &Password)
        -> Result<(), UserStoreError>;
}

#[derive(Debug, Error)]
pub enum UserStoreError {
    #[error("User already exists")]
    UserAlreadyExists,
    #[error("User not found")]
    UserNotFound,
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Incorrect credentials")]
    IncorrectCredentials,
    #[error("Unexpected error")]
    UnexpectedError(#[source] Report),
}

pub type ErrReport = color_eyre::Report; // or `eyre::Report`

impl From<ErrReport> for UserStoreError {
    fn from(err: ErrReport) -> Self {
        UserStoreError::UnexpectedError(err)
    }
}

impl From<String> for UserStoreError {
    fn from(err: String) -> Self {
        UserStoreError::UnexpectedError(Report::msg(err))
    }
}

impl PartialEq for UserStoreError {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::UserAlreadyExists, Self::UserAlreadyExists)
                | (Self::UserNotFound, Self::UserNotFound)
                | (Self::InvalidCredentials, Self::InvalidCredentials)
                | (Self::IncorrectCredentials, Self::IncorrectCredentials)
                | (Self::UnexpectedError(_), Self::UnexpectedError(_))
        )
    }
}

#[async_trait::async_trait]
pub trait BannedTokenStore: std::fmt::Debug {
    async fn ban_token(&mut self, token: Secret<String>) -> Result<(), BannedTokenStoreError>;
    async fn is_token_banned(&self, token: &Secret<String>) -> Result<bool, BannedTokenStoreError>;
}

#[derive(Debug, Error)]
pub enum BannedTokenStoreError {
    #[error("Unexpected error")]
    UnexpectedError(#[source] Report),
}

impl From<ErrReport> for BannedTokenStoreError {
    fn from(err: ErrReport) -> Self {
        BannedTokenStoreError::UnexpectedError(err)
    }
}

impl PartialEq for BannedTokenStoreError {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::UnexpectedError(_), Self::UnexpectedError(_))
        )
    }
}

// This trait represents the interface all concrete 2FA code stores should implement
#[async_trait::async_trait]
pub trait TwoFACodeStore: std::fmt::Debug {
    async fn add_code(
        &mut self,
        email: Email,
        login_attempt_id: LoginAttemptId,
        code: TwoFACode,
    ) -> Result<(), TwoFACodeStoreError>;
    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError>;
    async fn get_code(
        &self,
        email: &Email,
    ) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError>;
}

#[derive(Debug, Error)]
pub enum TwoFACodeStoreError {
    #[error("Login attempt ID not found")]
    LoginAttemptIdNotFound,
    #[error("2FA code not found")]
    TwoFACodeNotFound,
    #[error("Unexpected error")]
    UnexpectedError(#[source] Report),
}

impl From<ErrReport> for TwoFACodeStoreError {
    fn from(err: ErrReport) -> Self {
        TwoFACodeStoreError::UnexpectedError(err)
    }
}

impl From<String> for TwoFACodeStoreError {
    fn from(err: String) -> Self {
        TwoFACodeStoreError::UnexpectedError(Report::msg(err))
    }
}

impl From<TwoFACodeStoreError> for String {
    fn from(err: TwoFACodeStoreError) -> Self {
        match err {
            TwoFACodeStoreError::LoginAttemptIdNotFound => "Login attempt ID not found".to_owned(),
            TwoFACodeStoreError::TwoFACodeNotFound => "2FA code not found".to_owned(),
            TwoFACodeStoreError::UnexpectedError(e) => format!("Unexpected error: {}", e),
        }
    }
}

impl PartialEq for TwoFACodeStoreError {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::LoginAttemptIdNotFound, Self::LoginAttemptIdNotFound)
                | (Self::TwoFACodeNotFound, Self::TwoFACodeNotFound)
                | (Self::UnexpectedError(_), Self::UnexpectedError(_))
        )
    }
}

#[derive(Debug, Clone)]
pub struct LoginAttemptId(Secret<String>);

impl LoginAttemptId {
    pub fn parse(id: Secret<String>) -> Result<Self> {
        let parsed_id =
            uuid::Uuid::parse_str(id.expose_secret()).wrap_err("Invalid login attempt id")?;
        Ok(Self(Secret::new(parsed_id.to_string())))
    }
}

impl PartialEq for LoginAttemptId {
    fn eq(&self, other: &Self) -> bool {
        self.0.expose_secret() == other.0.expose_secret()
    }
}

impl Default for LoginAttemptId {
    fn default() -> Self {
        // Use the `uuid` crate to generate a random version 4 UUID
        Self(Secret::new(uuid::Uuid::new_v4().to_string()))
    }
}

// Implement AsRef<str> for LoginAttemptId
impl AsRef<Secret<String>> for LoginAttemptId {
    fn as_ref(&self) -> &Secret<String> {
        &self.0
    }
}

#[derive(Clone, Debug)]
pub struct TwoFACode(Secret<String>);

impl TwoFACode {
    pub fn parse(code: Secret<String>) -> Result<Self> {
        let exposed_code = code.expose_secret().trim().to_string();

        // Ensure `code` is a valid 6-digit code
        if exposed_code.len() == 6 && exposed_code.chars().all(|c| c.is_ascii_digit()) {
            Ok(Self(code))
        } else {
            Err(eyre!("Invalid 2FA code"))
        }
    }
}

impl PartialEq for TwoFACode {
    fn eq(&self, other: &Self) -> bool {
        self.0.expose_secret() == other.0.expose_secret()
    }
}

impl Default for TwoFACode {
    fn default() -> Self {
        // Use the `rand` crate to generate a random 2FA code.
        // The code should be 6 digits (ex: 834629)
        let code: String = (0..6)
            .map(|_| rand::random::<u8>() % 10) // Generate a random digit (0-9)
            .map(|digit| char::from(b'0' + digit)) // Convert digit to char
            .collect();
        Self(Secret::new(code))
    }
}

// Implement AsRef<str> for TwoFACode
impl AsRef<Secret<String>> for TwoFACode {
    fn as_ref(&self) -> &Secret<String> {
        &self.0
    }
}
