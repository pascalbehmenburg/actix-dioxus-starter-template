use actix_http::StatusCode;

use crate::util::error::Error;

#[derive(Debug, derive_more::Display)]

pub enum UserError {
  #[display(fmt = "Invalid email or password provided. Try again.")]
  InvalidEmailOrPassword,
}

impl From<UserError> for Error {
  fn from(error: UserError) -> Self {
    match error {
      UserError::InvalidEmailOrPassword => {
        Error::External(StatusCode::UNAUTHORIZED, error.to_string())
      }
    }
  }
}
