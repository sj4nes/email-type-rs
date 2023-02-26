use std::str::FromStr;

use fake::{Fake, Faker};
use fake::faker::internet::en::FreeEmail;

use crate::email::Email;

/// Return random `Email`
pub fn get_random_email() -> Email {
    let email_str = FreeEmail().fake::<String>();
    Email::from_str(&email_str).unwrap()
}
