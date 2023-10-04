use std::ops::{ControlFlow, FromResidual, Try};

use actix_http::body::EitherBody;
use actix_web::{
  http::header::ContentType, HttpRequest, HttpResponse, Responder,
  ResponseError,
};
use serde::Serialize;

use crate::util::body::JsonBody;
use crate::util::error::Error;

pub struct JsonResponse(pub Result<JsonBody, Error>);

impl Responder for JsonResponse {
  type Body = EitherBody<String>;

  fn respond_to(self, _: &HttpRequest) -> HttpResponse<Self::Body> {
    let result = self.0;
    match result {
      Ok(json_body) => match HttpResponse::Ok()
        .content_type(ContentType::json())
        .message_body(json_body.0.to_string())
      {
        Ok(res) => res.map_into_left_body(),
        Err(e) => e.error_response().map_into_right_body(),
      },
      Err(error) => error.error_response().map_into_right_body(),
    }
  }
}

impl<S: Serialize, E: Into<Error>> From<Result<S, E>> for JsonResponse {
  fn from(result: Result<S, E>) -> Self {
    match result {
      Ok(body) => match serde_json::to_value(body) {
        Ok(json_value) => Self(Ok(json_value.into())),
        Err(error) => Self(Err(Error::Other(error.into()))),
      },
      Err(error) => Self(Err(error.into())),
    }
  }
}

// Generic FromResidual Implementation for all types that implement Into<JsonBody>
// and Into<Error> which allows one to use the ? operator in the async functions
impl<O, E> FromResidual<std::result::Result<O, E>> for JsonResponse
where
  O: Into<JsonBody>,
  E: Into<Error>,
{
  fn from_residual(residual: std::result::Result<O, E>) -> Self {
    match residual {
      Ok(body) => Self(Ok(body.into())),
      Err(error) => Self(Err(error.into())),
    }
  }
}

impl Try for JsonResponse {
  type Output = JsonBody;
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
