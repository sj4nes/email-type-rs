use std::str::FromStr;

use log::error;
use serde::{Deserialize, Serialize, Serializer};
use validate::rules::email;

use crate::error::InvalidEmailError;

#[derive(Deserialize, PartialEq, Hash, Eq, Clone, Debug)]
pub struct Email(String);

impl Email {
    pub fn parse(s: &str) -> Result<Email, InvalidEmailError> {
        match email().validate(&s) {
            Ok(_) => Ok(Self(s.to_string())),

            Err(e) => {
                error!("invalid e-mail '{}': {}", s, e.get_message());
                Err(InvalidEmailError::ParseError)
            }
        }
    }
}

impl Serialize for Email {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl FromStr for Email {
    type Err = InvalidEmailError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let email = Email::parse(s)?;

        Ok(email)
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod email_tests {
    use fake::{Fake, Faker};
    use fake::faker::internet::en::FreeEmail;

    use crate::email::Email;

    #[test]
    fn return_ok_for_valid_email() {
        for _ in 1..30 {
            let email = FreeEmail().fake::<String>();
            assert!(Email::parse(&email).is_ok());
        }
    }

    #[test]
    fn return_error_for_invalid_email() {
        assert!(Email::parse("").is_err());

        for _ in 1..30 {
            let value = Faker.fake::<String>();
            assert!(Email::parse(&value).is_err());
        }
    }
}
