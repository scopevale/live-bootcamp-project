use color_eyre::eyre::{eyre, Result};
use secrecy::{ExposeSecret, Secret};

#[derive(Debug, Clone)]
pub struct Password(Secret<String>);

impl PartialEq for Password {
    fn eq(&self, other: &Self) -> bool {
        // We can use the expose_secret method to expose the secret in a
        // controlled manner when needed!
        self.0.expose_secret() == other.0.expose_secret() // Updated!
    }
}

impl Password {
    pub fn parse(s: Secret<String>) -> Result<Password> {
        if validate_password(&s) {
            Ok(Self(s))
        } else {
            Err(eyre!("Failed to parse string to a Password type".to_owned()))
        }
    }

    pub fn as_str(&self) -> &Secret<String> {
        &self.0
    }

    pub fn verify(&self, candidate: Secret<String>) -> bool {
        tracing::info!(
            "Verifying password: {} against {}",
            &candidate.expose_secret(),
            &self.0.expose_secret()
        );
        self.0.expose_secret() == candidate.expose_secret()
    }
}

fn validate_password(s: &Secret<String>) -> bool {
    s.expose_secret().len() >= 8
}

impl AsRef<Secret<String>> for Password {
    fn as_ref(&self) -> &Secret<String> {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::Password;

    use fake::faker::internet::en::Password as FakePassword;
    use fake::Fake;
    use secrecy::Secret;

    #[test]
    fn empty_string_is_rejected() {
        let password = Secret::new("".to_owned());
        assert!(Password::parse(password).is_err());
    }
    #[test]
    fn string_less_than_8_characters_is_rejected() {
        let password = Secret::new("1234567".to_owned());
        assert!(Password::parse(password).is_err());
    }

    #[derive(Debug, Clone)]
    struct ValidPasswordFixture(pub Secret<String>);

    impl quickcheck::Arbitrary for ValidPasswordFixture {
        fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
            let password = FakePassword(8..30).fake_with_rng(g);
            Self(Secret::new(password))
        }
    }
    #[quickcheck_macros::quickcheck]
    fn valid_passwords_are_parsed_successfully(valid_password: ValidPasswordFixture) -> bool {
        Password::parse(valid_password.0).is_ok()
    }

    #[test]
    fn incorrect_passwords_are_not_verified() {
        let password = Secret::new("correcthorsebatterystaple".to_owned());
        let password = Password::parse(password).unwrap();
        assert!(!password.verify(Secret::new("wrongpassword".to_owned())));
    }

    #[test]
    fn correct_passwords_are_verified() {
        let password = Secret::new("correcthorsebatterystaple".to_owned());
        let password = Password::parse(password.clone()).unwrap();
        assert!(password.verify(password.0.to_owned()));
    }
}
