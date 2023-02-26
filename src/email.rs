use std::ops::Deref;
use std::str::FromStr;

use log::error;
use serde::{Deserialize, Serialize, Serializer};
use validate::rules::email;

use crate::error::InvalidEmailError;

/// Email type for type-driven design, also based on
/// approach [Parse, don't validate](https://lexi-lambda.github.io/blog/2019/11/05/parse-don-t-validate/)
///
/// ## Example:
///
/// ```rust
/// use std::str::FromStr;
/// use email_type_rs::email::Email;
///
/// match Email::from_str("lexi.lambda@gmail.com") {
///     Ok(email) => println!("email: {}", email.as_ref()),
///     Err(e) => eprintln!("{}", e)
/// }
/// ```
///
#[derive(Deserialize, PartialEq, Hash, Eq, Clone, Debug)]
#[serde(try_from = "String", into = "String")]
pub struct Email(String);

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
        match email().validate(&s) {
            Ok(_) => Ok(Self(s.to_string())),

            Err(e) => {
                error!("invalid e-mail '{}': {}", s, e.get_message());
                Err(InvalidEmailError::ParseError)
            }
        }
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for Email {
    type Error = InvalidEmailError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Email::from_str(&value)
    }
}

impl From<Email> for String {
    fn from(value: Email) -> Self {
        value.0
    }
}

impl Deref for Email {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use fake::{Fake, Faker};
    use fake::faker::internet::en::FreeEmail;
    use serde::{Deserialize, Serialize};

    use crate::email::Email;

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct User {
        pub email: Email
    }

    #[test]
    fn return_ok_for_valid_email() {
        for _ in 1..30 {
            let email = FreeEmail().fake::<String>();
            assert!(Email::from_str(&email).is_ok());
        }
    }

    #[test]
    fn return_error_for_invalid_email() {
        assert!(Email::from_str("").is_err());

        for _ in 1..30 {
            let value = get_random_string();
            assert!(Email::from_str(&value).is_err());
        }
    }

    #[test]
    fn serialization_deserialization_test_for_invalid_value() {
        let json = "{\"email\":\"invalid-email\"}".to_string();
        match serde_json::from_str::<User>(&json) {
            Ok(_) => panic!("error expected"),
            Err(e) => println!("{}", e)
        }
    }

    #[test]
    fn use_as_str() {
        let email = Email::from_str("a@b.com").unwrap();
        assert_str_func(&email);
    }

    fn get_random_string() -> String {
        Faker.fake::<String>()
    }

    fn assert_str_func(_: &str) {
        assert!(true)
    }
}
