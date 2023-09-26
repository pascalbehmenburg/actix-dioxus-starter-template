use actix_web::{error, http::StatusCode};

#[allow(dead_code)]
#[derive(derive_more::From, Debug, derive_more::Display)]
pub enum Error {
  // this basically catches everything that isn't catched already so this error is universally usable
  #[from]
  #[display(fmt = "Internal unspecified error occurred: {:?}", _0)]
  UnspecificInternalError(anyhow::Error),

  // Add whatever errors suits your needs:
  #[from]
  #[display(fmt = "Database error occurred: {:?}", _0)]
  SqlxError(sqlx::Error),

  #[from]
  #[display(fmt = "Argon2 hashing error occurred: {:?}", _0)]
  Argon2HashingError(argon2::Error),

  #[from]
  #[display(fmt = "Argon2 hashing error occurred: {:?}", _0)]
  Argon2PasswordHashingError(argon2::password_hash::Error),

  #[from]
  #[display(fmt = "Serialization error occurred: {:?}", _0)]
  SerializationError(serde_json::Error),

  #[from]
  #[display(fmt = "Actix Web Server error occurred: {:?}", _0)]
  ActixWebServerError(actix_web::error::Error),

  // This produces custom HTTP errors for example after a failed login attempt:
  // Error 401: wrong login credentials
  #[display(fmt = "Error {:?}: {:?}", _0, _1)]
  CustomHTTPResponse(StatusCode, String),
}

impl Error {
  pub fn log(&self) {
    tracing::error!("[{:?}] {:?}", self.http_status_code(), self);
  }

  // provide the mappings from internal error specifiers to HTTP status codes
  // this enables one to implement ResponseError and therefore be able to propagate all errors
  // to the user in a standardized way
  pub fn http_status_code(&self) -> StatusCode {
    match self {
      Error::CustomHTTPResponse(status_code, _) => *status_code,
      Error::ActixWebServerError(e) => e.as_response_error().status_code(),
      _ => StatusCode::INTERNAL_SERVER_ERROR,
    }
  }
}

impl error::ResponseError for Error {
  // use standard implementation of error_response since it uses the internal display
  // function and the status code function below so it is provided with everything we need
  fn status_code(&self) -> StatusCode {
    self.log();
    self.http_status_code()
  }
}

impl std::error::Error for Error {}
