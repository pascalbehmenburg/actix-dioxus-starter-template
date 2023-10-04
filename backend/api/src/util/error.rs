use actix_http::StatusCode;
use actix_web::{http::header::ContentType, HttpResponse, ResponseError};
use color_eyre::eyre;

#[derive(Debug, derive_more::Display)]
/// External variant is used to display a status and message to the user. It does not log the error.
/// Refer to `with_error_log` or `with_debug_log` for logging the error.
///
/// Internal variant is used to display a status to the user and log the error.
///
/// Other variant is used to display an InternalServerError to the user and log the error.
/// Also used for converting other errors into an Error without explicit conversions.
pub enum Error {
  #[display(fmt = "Error {}: {}", _0, _1)]
  External(StatusCode, String),

  #[display(fmt = "Status: {}\n{}", _0, _1)]
  Internal(StatusCode, eyre::Error),

  #[display(fmt = "{}", _0)]
  Other(eyre::Error),
}

#[allow(dead_code)]
impl Error {
  fn with_error_log<T: Into<eyre::Error>>(&self, source: T) {
    let e: eyre::Error = source.into();
    tracing::error!("{:?}", e);
  }

  fn with_debug_log<T: Into<eyre::Error>>(&self, source: T) {
    let e: eyre::Error = source.into();
    tracing::debug!("{:?}", e);
  }
}

impl ResponseError for Error {
  fn status_code(&self) -> StatusCode {
    match self {
      Error::External(status_code, _) => *status_code,
      Error::Internal(status_code, _) => *status_code,
      Error::Other(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
  }

  fn error_response(&self) -> HttpResponse {
    match self {
      Error::External(status_code, error_message) => {
        HttpResponse::build(*status_code)
          .content_type(ContentType::plaintext())
          .body((*error_message).clone())
      }
      Error::Internal(status_code, error) => {
        tracing::error!("{:?}", error);
        HttpResponse::build(*status_code)
          .content_type(ContentType::plaintext())
          .body(format!("Status {}: Something must've wen't wrong on our end, we're working on it.", status_code))
      }
      Error::Other(error) => {
        tracing::error!("{:?}", error);
        HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
          .content_type(ContentType::plaintext())
          .body(format!("Status {}: We did a little wunky funky and I cannot tell you what it is.", StatusCode::INTERNAL_SERVER_ERROR))
      }
    }
  }
}

impl<T: Into<eyre::Error>> From<T> for Error {
  fn from(e: T) -> Self {
    Error::Other(e.into())
  }
}
