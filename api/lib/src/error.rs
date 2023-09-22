use std::fmt::Display;

use actix_identity::error::LoginError;
use actix_web::{
  error,
  http::{header::ContentType, StatusCode},
  HttpResponse,
};
use derive_more::Error;

use crate::response::ApiData;

#[derive(Debug, Error, Clone)]
pub struct ApiError {
  pub error_kind: ApiErrorKind,
  pub debug_info: String,
}

impl Display for ApiError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let error_kind = self.error_kind.to_string();
    let debug_info = self.debug_info.to_string();
    write!(f, "error_kind: {}\n debug_info: {}", error_kind, debug_info)
  }
}

#[allow(dead_code)]
#[derive(Debug, derive_more::Display, Clone)]
pub enum ApiErrorKind {
  #[display(
    fmt = "Bad Request: The server cannot or will not process your reequest \
                due to something that is perceived to be a client error."
  )]
  BadRequest,
  #[display(
    fmt = "Unauthorized: Authentication is required and has failed or has not yet been provided. \
                Please provide valid login credentials and try again."
  )]
  Unauthorized,
  #[display(
    fmt = "Forbidden: You do not have permission to access this resource."
  )]
  Forbidden,
  #[display(fmt = "Not Found: The requested resource could not be found.")]
  NotFound,
  #[display(
    fmt = "Conflict: The request could not be completed due to a conflict \
                with the current state of the target resource. Please wait a moment and try again."
  )]
  Conflict,
  #[display(fmt = "Unprocessable Entity: The request was well-formed but \
                was unable to be followed due to semantic errors. Please check your request and try again.")]
  UnprocessableEntity,
  #[display(
    fmt = "Internal Server Error: The server encountered an unexpected condition \
                that prevented it from fulfilling the request."
  )]
  InternalServerError,
  #[display(
    fmt = "Service Unavailable: The server is currently unable to handle the request \
                due to a temporary overload or scheduled maintenance. Please try again later."
  )]
  ServiceUnavailable,
  #[display(
    fmt = "Database Error: The server encountered an unexpected condition \
                that prevented it from fulfilling the request."
  )]
  RepositoryError,
  #[display(
    fmt = "Hashing Error: The server encountered an unexpected condition \
                that prevented it from fulfilling the request."
  )]
  HashingError,
  #[display(
    fmt = "Serialization Error: The server encountered an unexpected condition \
                that prevented it from fulfilling the request."
  )]
  SerializationError,
  #[display(
    fmt = "Internal Web Server Lib Error: The web server encountered an unexpected condition \
                    that prevented it from fulfilling the request."
  )]
  ActixWebServerError,
}

impl error::ResponseError for ApiError {
  fn error_response(&self) -> HttpResponse {
    self.to_http_response()
  }

  fn status_code(&self) -> StatusCode {
    self.http_status_code()
  }
}
// TODO: move status_code and to impl ApiError
impl ApiError {
  pub fn http_status_code(&self) -> StatusCode {
    match self.error_kind {
      ApiErrorKind::BadRequest => StatusCode::BAD_REQUEST,
      ApiErrorKind::Unauthorized => StatusCode::UNAUTHORIZED,
      ApiErrorKind::Forbidden => StatusCode::FORBIDDEN,
      ApiErrorKind::NotFound => StatusCode::NOT_FOUND,
      ApiErrorKind::Conflict => StatusCode::CONFLICT,
      ApiErrorKind::UnprocessableEntity => StatusCode::UNPROCESSABLE_ENTITY,
      ApiErrorKind::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
      ApiErrorKind::ServiceUnavailable => StatusCode::SERVICE_UNAVAILABLE,
      ApiErrorKind::RepositoryError => StatusCode::INTERNAL_SERVER_ERROR,
      ApiErrorKind::HashingError => StatusCode::INTERNAL_SERVER_ERROR,
      ApiErrorKind::SerializationError => StatusCode::INTERNAL_SERVER_ERROR,
      ApiErrorKind::ActixWebServerError => StatusCode::INTERNAL_SERVER_ERROR,
    }
  }

  pub fn to_http_response(&self) -> HttpResponse {
    tracing::debug!("{:?}", self.error_kind.to_string());
    tracing::debug!("{:?}", self.debug_info);
    HttpResponse::build(self.http_status_code())
      .insert_header(ContentType::html())
      .body(self.error_kind.to_string())
  }

  pub fn to_http_response_with_data(&self) -> HttpResponse<ApiData> {
    tracing::debug!("{:?}", self.error_kind.to_string());
    tracing::debug!("{:?}", self.debug_info);
    HttpResponse::build(self.http_status_code())
      .insert_header(ContentType::json())
      .message_body(<ApiData as From<ApiError>>::from(self.clone()))
      .unwrap()
  }
}

impl From<ApiError> for ApiData {
  fn from(e: ApiError) -> Self {
    ApiData(Some(serde_json::json!({
      "error": e.error_kind.to_string(),
      "debug_info": e.debug_info,
    })))
  }
}

impl From<ApiError> for HttpResponse {
  fn from(e: ApiError) -> Self {
    e.to_http_response()
  }
}

impl From<sqlx::Error> for ApiError {
  fn from(e: sqlx::Error) -> Self {
    ApiError {
      error_kind: ApiErrorKind::RepositoryError,
      debug_info: e.to_string(),
    }
  }
}

impl From<argon2::password_hash::Error> for ApiError {
  fn from(e: argon2::password_hash::Error) -> Self {
    ApiError {
      error_kind: ApiErrorKind::HashingError,
      debug_info: e.to_string(),
    }
  }
}

impl From<argon2::Error> for ApiError {
  fn from(e: argon2::Error) -> Self {
    ApiError {
      error_kind: ApiErrorKind::HashingError,
      debug_info: e.to_string(),
    }
  }
}

impl From<serde_json::Error> for ApiError {
  fn from(e: serde_json::Error) -> Self {
    ApiError {
      error_kind: ApiErrorKind::SerializationError,
      debug_info: e.to_string(),
    }
  }
}

impl From<actix_web::error::Error> for ApiError {
  fn from(e: actix_web::error::Error) -> Self {
    ApiError {
      error_kind: ApiErrorKind::ActixWebServerError,
      debug_info: e.to_string(),
    }
  }
}

impl From<LoginError> for ApiError {
  fn from(e: LoginError) -> Self {
    ApiError {
      error_kind: ApiErrorKind::Unauthorized,
      debug_info: e.to_string(),
    }
  }
}
