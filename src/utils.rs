use fake::Faker;
use fake::faker::internet::en::FreeEmail;

use crate::email::Email;

/// Return random `Email`
pub fn get_random_email() -> Email {
    let email_str = FreeEmail().fake::<String>();
    Email::parse(email_str).unwrap()
}