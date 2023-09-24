use std::ops::{ControlFlow, Deref, FromResidual, Try};

use actix_http::body::EitherBody;
use actix_web::{
  http::header::ContentType, HttpRequest, HttpResponse, Responder,
  ResponseError,
};
use serde::Serialize;

use crate::util::body::Body;
use crate::util::error::Error;
pub struct Response(pub Result<Body, Error>);

impl AsRef<Result<Body, Error>> for Response {
  fn as_ref(&self) -> &Result<Body, Error> {
    &self.0
  }
}

impl Deref for Response {
  type Target = Result<Body, Error>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl Responder for Response {
  type Body = EitherBody<String>;

  fn respond_to(self, _: &HttpRequest) -> HttpResponse<Self::Body> {
    let result = self.as_ref();
    match result {
      Ok(json_body) => match HttpResponse::Ok()
        .content_type(ContentType::json())
        .message_body(json_body.as_ref().to_string())
      {
        Ok(res) => res.map_into_left_body(),
        Err(e) => Error::ActixWebServerError(e)
          .error_response()
          .map_into_right_body(),
      },
      Err(error) => error.error_response().map_into_right_body(),
    }
  }
}

// Since any serialization could yield an error we cannot implement a conversion
// from any S: Serialize to JsonBody without having to deal with the possibility
// of an error. Therefore we implement a conversion from Result<S, E> where E
// implements Into<Error> to JsonResponse. This allows us to use the ? operator
// in the async functions of the repositories.
impl<S: Serialize, E: Into<Error>> From<Result<S, E>> for Response {
  fn from(result: Result<S, E>) -> Self {
    match result {
      Ok(body) => match serde_json::to_value(body) {
        Ok(json_value) => Self(Ok(json_value.into())),
        Err(error) => Self(Err(error.into())),
      },
      Err(error) => Self(Err(error.into())),
    }
  }
}

// Generic FromResidual Implementation for all types that implement Into<JsonBody>
// and Into<Error> which allows one to use the ? operator in the async functions
impl<O, E> FromResidual<std::result::Result<O, E>> for Response
where
  O: Into<Body>,
  E: Into<Error>,
{
  fn from_residual(residual: std::result::Result<O, E>) -> Self {
    match residual {
      Ok(body) => Self(Ok(body.into())),
      Err(error) => Self(Err(error.into())),
    }
  }
}

impl Try for Response {
  type Output = Body;
  type Residual = Result<std::convert::Infallible, Error>;

  fn from_output(output: Self::Output) -> Self {
    Self(Ok(output))
  }

  fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
    match self.0 {
      Ok(body) => ControlFlow::Continue(body),
      Err(error) => ControlFlow::Break(Err(error)),
    }
  }
}
