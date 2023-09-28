use std::{convert::Infallible, ops::Deref};

// Supports the Response implmentation of this crate by wrapping the Value type
// and therefore allowing us to implement infallible and type conversions
// so that Response can implement the Try trait which gives us cleaner code and
// unified error handling.
pub struct JsonBody(pub serde_json::Value);

// some helper functions so that one does not have to use .0 syntax
// which I find to be ugly and unexpressive
// but instead is able to use .as_ref() or .deref() to access the inner value
impl AsRef<serde_json::Value> for JsonBody {
  fn as_ref(&self) -> &serde_json::Value {
    &self.0
  }
}

impl Deref for JsonBody {
  type Target = serde_json::Value;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

// This conversion is used to support the Try implementation in JsonResponse
impl From<serde_json::Value> for JsonBody {
  fn from(value: serde_json::Value) -> Self {
    Self(value)
  }
}

// Infallible is a type that can never be instantiated, and is used to indicate
// that a function can never return. This is useful for match arms that are
// impossible to reach, but are required to be present by the compiler.
// https://doc.rust-lang.org/std/convert/enum.Infallible.html
// Furthermore an Infallible type implementation is required for the Try trait
// see https://doc.rust-lang.org/std/ops/trait.Try.html
// also see JsonResponse
impl From<Infallible> for JsonBody {
  fn from(err: Infallible) -> Self {
    unreachable!(
      "Something tried to instantiate JsonBody via Infallible: {:?}",
      err
    )
  }
}
