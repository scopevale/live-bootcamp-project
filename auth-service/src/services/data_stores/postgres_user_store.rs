use color_eyre::eyre::{Context, Result};

use argon2::{
    password_hash::rand_core::OsRng, password_hash::SaltString, Algorithm, Argon2, Params,
    PasswordHash, PasswordHasher, PasswordVerifier, Version,
};

use secrecy::{ExposeSecret, Secret};
use sqlx::PgPool;

use crate::{
    domain::{Email, Password, User},
    services::data_stores::{UserStore, UserStoreError},
};

#[derive(Debug, Clone)]
pub struct PostgresUserStore {
    pool: PgPool,
}

impl PostgresUserStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl UserStore for PostgresUserStore {
    // Implement all required methods. Note that you will need to make SQL queries against
    // our PostgreSQL instance inside these methods.

    #[tracing::instrument(name = "Adding user to PostgreSQL", skip_all)]
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        let password_hash = compute_password_hash(user.password.as_ref().to_owned())
            .await
            .map_err(UserStoreError::UnexpectedError)?;
        let result = sqlx::query!(
            r#"INSERT INTO users (email, password_hash, requires_2fa) VALUES ($1, $2, $3)"#,
            user.email.as_ref().expose_secret(),
            &password_hash.expose_secret(),
            user.requires_2fa
        )
        .execute(&self.pool)
        .await;

        match result {
            Ok(_) => Ok(()),
            Err(sqlx::Error::Database(db_err)) if db_err.code() == Some("23505".into()) => {
                // 23505 is the PostgreSQL error code for unique_violation
                Err(UserStoreError::UserAlreadyExists)
            }
            Err(e) => Err(UserStoreError::UnexpectedError(e.into())),
        }
    }

    #[tracing::instrument(name = "Retrieving user from PostgreSQL", skip_all)]
    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
        sqlx::query!(
            r#"SELECT email, password_hash, requires_2fa FROM users WHERE email = $1"#,
            email.as_ref().expose_secret()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| UserStoreError::UnexpectedError(e.into()))?
        .map(|row| {
            Ok(User {
                email: Email::parse(Secret::new(row.email))
                    .map_err(UserStoreError::UnexpectedError)?,
                password: Password::parse(Secret::new(row.password_hash))
                    .map_err(UserStoreError::UnexpectedError)?,
                requires_2fa: row.requires_2fa,
            })
        })
        .ok_or(UserStoreError::UserNotFound)?
    }

    #[tracing::instrument(name = "Validating user credentials in PostgreSQL", skip_all)]
    async fn validate_user(
        &self,
        email: &Email,
        password: &Password,
    ) -> Result<(), UserStoreError> {
        let user = self.get_user(email).await?;

        verify_password_hash(
            user.password.as_ref().to_owned(),
            password.as_ref().to_owned(),
        )
        .await
        .map_err(|_| UserStoreError::IncorrectCredentials)
    }
}

// Helper function to verify if a given password matches an expected hash
// Hashing is a CPU-intensive operation. To avoid blocking
// other async tasks, update this function to perform hashing on a
// separate thread pool using tokio::task::spawn_blocking. Note that you
// will need to update the input parameters to be String types instead of &str
#[tracing::instrument(name = "Verify password hash", skip_all)]
async fn verify_password_hash(
    expected_password_hash: Secret<String>,
    password_candidate: Secret<String>,
) -> Result<()> {
    let current_span: tracing::Span = tracing::Span::current();

    let result = tokio::task::spawn_blocking(move || {
        current_span.in_scope(|| {
            let password_candidate = password_candidate.expose_secret().as_bytes();

            let expected_password_hash: PasswordHash<'_> =
                PasswordHash::new(expected_password_hash.expose_secret())?;

            tracing::debug!(
                ?expected_password_hash.hash,
                ?password_candidate,
                "debugging inside spawn_blocking"
            );

            tracing::debug!(?expected_password_hash, "parsed expected hash");
            tracing::debug!(?password_candidate, "password candidate");

            Argon2::default()
                .verify_password(password_candidate, &expected_password_hash)
                .wrap_err("failed to verify password hash")
        })
    })
    .await;

    tracing::debug!(?result, "result from spawn_blocking");

    result?
}

// Helper function to hash passwords before persisting them in the database.
// Hashing is a CPU-intensive operation. To avoid blocking
// other async tasks, update this function to perform hashing on a
// separate thread pool using tokio::task::spawn_blocking. Note that you
// will need to update the input parameters to be String types instead of &str
#[tracing::instrument(name = "Computing password hash", skip_all)]
async fn compute_password_hash(password: Secret<String>) -> Result<Secret<String>> {
    let current_span: tracing::Span = tracing::Span::current();

    let result = tokio::task::spawn_blocking(move || {
        current_span.in_scope(|| {
            let mut rng = OsRng;
            let salt: SaltString = SaltString::generate(&mut rng);
            let password_hash = Argon2::new(
                Algorithm::Argon2id,
                Version::V0x13,
                Params::new(15000, 2, 1, None)?,
            )
            .hash_password(password.expose_secret().as_bytes(), &salt)?
            .to_string();

            Ok(Secret::new(password_hash))
        })
    })
    .await;

    result?
}
