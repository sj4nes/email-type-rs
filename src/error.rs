use thiserror::Error;

#[derive(Error, Debug)]
pub enum InvalidEmailError {
    #[error("Parse error")]
    ParseError
}