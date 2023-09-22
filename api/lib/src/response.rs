use std::{
  convert::Infallible,
  ops::{FromResidual, Try},
};

use crate::error::ApiError;
use actix_web::{
  body::{BodySize, MessageBody},
  http::header::ContentType,
  HttpResponse, Responder,
};
use serde::Serialize;

#[derive(Debug)]
pub struct ApiResponse(pub Result<ApiData, ApiError>);

impl<S: Serialize> From<S> for ApiResponse {
  fn from(s: S) -> Self {
    match serde_json::to_value(s) {
      Ok(json) => Self(Ok(ApiData(Some(json)))),
      Err(e) => Self(Err(e.into())),
    }
  }
}

impl<O, E> FromResidual<std::result::Result<O, E>> for ApiResponse
where
  O: Into<ApiData>,
  E: Into<ApiError>,
{
  fn from_residual(residual: std::result::Result<O, E>) -> Self {
    match residual {
      Ok(o) => Self(Ok(o.into())),
      Err(e) => Self(Err(e.into())),
    }
  }
}

impl From<Infallible> for ApiData {
  fn from(_: Infallible) -> Self {
    Self(None)
  }
}

impl Try for ApiResponse {
  type Output = ApiData;

  type Residual = Result<std::convert::Infallible, ApiError>;

  fn from_output(output: Self::Output) -> Self {
    Self(Ok(output))
  }

  fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
    match self {
      Self(Ok(o)) => std::ops::ControlFlow::Continue(o),
      Self(Err(e)) => std::ops::ControlFlow::Break(Err(e)),
    }
  }
}

#[derive(Debug)]
pub struct ApiData(pub Option<serde_json::Value>);

impl From<serde_json::Value> for ApiData {
  fn from(json: serde_json::Value) -> Self {
    Self(Some(json))
  }
}

impl From<ApiData> for HttpResponse<ApiData> {
  fn from(api_data: ApiData) -> Self {
    match api_data.0 {
      Some(json) => HttpResponse::Ok()
        .message_body(<serde_json::Value as Into<ApiData>>::into(json))
        .unwrap(),
      None => HttpResponse::Ok().message_body(ApiData(None)).unwrap(),
    }
  }
}

impl MessageBody for ApiData {
  type Error = ApiError;

  fn size(&self) -> actix_web::body::BodySize {
    match &self.0 {
      Some(json) => BodySize::Sized(json.to_string().len() as u64),
      None => BodySize::None,
    }
  }

  fn poll_next(
    self: std::pin::Pin<&mut Self>,
    _cx: &mut std::task::Context<'_>,
  ) -> std::task::Poll<Option<Result<actix_web::web::Bytes, Self::Error>>> {
    let api_data = self.get_mut();
    //TODO
    // for now just send all json data later on one could find a way to distingiush between
    // single json objects and arrays of json objects
    // so that one can stream the arrays of json objects instead of sending them as a whole
    let result = std::task::Poll::Ready(match &api_data.0 {
      Some(json) => Some(Ok(actix_web::web::Bytes::from(json.to_string()))),
      None => None,
    });
    // replace data with None so that poll_next returns None and therefore the 'stream' is finished
    drop(std::mem::replace(api_data, ApiData(None)));
    result
  }
}

impl Responder for ApiResponse {
  type Body = ApiData;

  fn respond_to(
    self,
    _req: &actix_web::HttpRequest,
  ) -> HttpResponse<Self::Body> {
    // TODO: fix shit above then
    match self {
      Self(Ok(api_data)) => HttpResponse::Ok()
        .content_type(ContentType::json())
        .message_body(api_data)
        .unwrap(),
      Self(Err(api_error)) => api_error.to_http_response_with_data(),
    }
  }
}
