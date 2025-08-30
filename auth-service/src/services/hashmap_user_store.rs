use std::collections::HashMap;

// use axum::async_trait;

use crate::domain::{Email, Password, User, UserStore, UserStoreError};

// Create a new struct called `HashmapUserStore` containing a `users` field
// which stores a `HashMap`` of email `String`s mapped to `User` objects.
// Derive the `Default` trait for `HashmapUserStore`.
#[derive(Default)]
pub struct HashmapUserStore {
    users: HashMap<Email, User>,
}

#[async_trait::async_trait]
impl UserStore for HashmapUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        // Return `UserStoreError::UserAlreadyExists` if the user already exists,
        // otherwise insert the user into the hashmap and return `Ok(())`.
        if self.users.contains_key(&user.email) {
            return Err(UserStoreError::UserAlreadyExists);
        }
        self.users.insert(user.email.clone(), user);
        Ok(())
    }

    // Implement a public method called `get_user`, which takes an
    // immutable reference to self and an email string slice as arguments.
    // This function should return a `Result` type containing either a
    // `User` object or a `UserStoreError`.
    // Return `UserStoreError::UserNotFound` if the user can not be found.
    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
        // get the user from the hashmap
        match self.users.get(email) {
            Some(user) => Ok(user.clone()),
            None => Err(UserStoreError::UserNotFound),
        }
    }

    // Implement a public method called `validate_user`, which takes an
    // immutable reference to self, an email string slice, and a password string slice
    // as arguments. `validate_user` should return a `Result` type containing either a
    // unit type `()` if the email/password passed in match an existing user, or a `UserStoreError`.
    // Return `UserStoreError::UserNotFound` if the user can not be found.
    // Return `UserStoreError::InvalidCredentials` if the password is incorrect.
    async fn validate_user(&self, email: &Email, password: &Password)
        -> Result<(), UserStoreError> {
          match self.users.get(email) {
              Some(user) => {
                  if user.password.eq(password) {
                      Ok(())
                  } else {
                      Err(UserStoreError::InvalidCredentials)
                  }
              }
              None => Err(UserStoreError::UserNotFound),
          }
    }
}

// Add unit tests for your `HashmapUserStore` implementation
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_user() {
        let mut store = HashmapUserStore::default();
        let user = User {
            email: Email::parse("me@example.com".to_owned()).unwrap(),
            password: Password::parse("password123".to_owned()).unwrap(),
            requires_2fa: false,
        };

        // Add user for the first time
        let result = store.add_user(user.clone()).await;
        assert_eq!(result, Ok(()));

        // Try to add the same user again
        let result = store.add_user(user).await;
        assert_eq!(result, Err(UserStoreError::UserAlreadyExists));
    }

    #[tokio::test]
    async fn test_get_user() {
        let mut store = HashmapUserStore::default();
        let email = Email::parse("me@example.com".to_owned()).unwrap();
        let user = User {
            email: email.clone(),
            password: Password::parse("password123".to_owned()).unwrap(),
            requires_2fa: false,
        };

        // Get existing user
        store.users.insert(email.clone(), user.clone());
        let result = store.get_user(&email).await;
        assert_eq!(result, Ok(user.clone()));

        // Get non-existing user
        let result = store.get_user(&Email::parse("none@example.com".to_owned()).unwrap()).await;
        assert_eq!(result, Err(UserStoreError::UserNotFound));
    }

    #[tokio::test]
    async fn test_validate_user() {
        let mut store = HashmapUserStore::default();
        let email = Email::parse("me@example.com".to_owned()).unwrap();
        let password = Password::parse("password123".to_owned()).unwrap();
        let user = User {
            email: email.clone(),
            password: password.clone(),
            requires_2fa: false,
        };

        // add a new user for the tests
        store.users.insert(email.clone(), user.clone());

        // Validate with correct credentials
        let result = store.validate_user(&email, &password).await;
        assert_eq!(result, Ok(()));

        // Validate with incorrect password
        let incorrect_password = Password::parse("wrongpassword".to_owned()).unwrap();
        let result = store.validate_user(&user.email, &incorrect_password).await;
        assert_eq!(result, Err(UserStoreError::InvalidCredentials));

        // Validate with non-existing user
        let result = store
            .validate_user(
                &Email::parse("none@example.com".to_owned()).unwrap(),
                &password
            )
            .await;
        assert_eq!(result, Err(UserStoreError::UserNotFound));
    }
}
