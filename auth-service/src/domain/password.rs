#[derive(Debug, Clone, PartialEq)]
pub struct Password(String);

impl Password {
    pub fn parse(s: String) -> Result<Password, String> {
        if validate_password(&s) {
            Ok(Self(s))
        } else {
            Err("Failed to parse string to a Password type".to_owned())
        }
    }
}

fn validate_password(s: &str) -> bool {
    s.len() >= 8
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
