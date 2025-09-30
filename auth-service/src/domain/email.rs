// use std::hash::Hash;
//
// use color_eyre::eyre::{eyre, Result};
// use secrecy::{ExposeSecret, Secret};
// use validator::validate_email;
//
// #[derive(Debug, Clone)]
// pub struct Email(Secret<String>);
//
// impl PartialEq for Email {
//     fn eq(&self, other: &Self) -> bool {
//         self.0.expose_secret() == other.0.expose_secret()
//     }
// }
//
// impl Hash for Email {
//     fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
//         self.0.expose_secret().hash(state);
//     }
// }
//
// impl Eq for Email {}
//
// impl Email {
//     pub fn parse(s: Secret<String>) -> Result<Email> {
//         if validate_email(s.expose_secret()) {
//             Ok(Self(s))
//         } else {
//             Err(eyre!(format!(
//                 "{} is not a valid email.",
//                 s.expose_secret()
//             )))
//         }
//     }
// }
//
// impl AsRef<Secret<String>> for Email {
//     fn as_ref(&self) -> &Secret<String> {
//         &self.0
//     }
// }

use color_eyre::eyre::{eyre, Result};
use std::fmt;
use validator::ValidateEmail;

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct Email(String);

impl Email {
    pub fn parse(s: String) -> Result<Email> {
        if (s).validate_email() {
            Ok(Self(s))
        } else {
            Err(eyre!(format!("{} is not a valid email.", s)))
        }
    }
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::Email;

    use fake::faker::internet::en::SafeEmail;
    use fake::Fake;

    #[test]
    fn empty_string_is_rejected() {
        let email = "".to_string();
        assert!(Email::parse(email).is_err());
    }
    #[test]
    fn email_missing_at_symbol_is_rejected() {
        let email = "domain.com".to_string();
        assert!(Email::parse(email).is_err());
    }
    #[test]
    fn email_missing_username_is_rejected() {
        let email = "@domain.com".to_string();
        assert!(Email::parse(email).is_err());
    }
    #[test]
    fn email_missing_domain_is_rejected() {
        let email = "user@".to_string();
        assert!(Email::parse(email).is_err());
    }
    #[test]
    fn email_invalid_character_in_username_is_rejected() {
        let email = "bad>user@domain.com".to_string();
        assert!(Email::parse(email).is_err());
    }
    #[test]
    fn email_invalid_character_in_domain_is_rejected() {
        let email = "gooduser@bad&domain.com".to_string();
        assert!(Email::parse(email).is_err());
    }

    #[derive(Debug, Clone)]
    struct ValidEmailFixture(pub String);

    impl quickcheck::Arbitrary for ValidEmailFixture {
        fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
            let email = SafeEmail().fake_with_rng(g);
            Self(email)
        }
    }

    #[quickcheck_macros::quickcheck]
    fn valid_emails_are_parsed_successfully(valid_email: ValidEmailFixture) -> bool {
        Email::parse(valid_email.0).is_ok()
    }
}
