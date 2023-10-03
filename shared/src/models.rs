use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
#[derive(
  Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default,
)]
pub struct Todo {
  pub id: i64,
  pub title: String,
  pub description: String,
  pub is_done: bool,
  pub owner: i64,
  pub created_at: chrono::DateTime<chrono::Utc>,
  pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(
  Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default,
)]
pub struct CreateTodo {
  pub title: String,
  pub description: String,
}

#[derive(
  Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default,
)]
pub struct UpdateTodo {
  pub id: i64,
  pub title: Option<String>,
  pub description: Option<String>,
  pub is_done: Option<bool>,
}

#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
#[derive(
  Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default,
)]
pub struct User {
  pub id: i64,
  pub name: String,
  pub email: String,
  pub password: String,
  pub created_at: chrono::DateTime<chrono::Utc>,
  pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(
  Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default,
)]
pub struct CreateUser {
  pub name: String,
  pub email: String,
  pub password: String,
}

#[derive(
  Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default,
)]
pub struct UpdateUser {
  pub name: Option<String>,
  pub email: Option<String>,
  pub password: Option<String>,
}

#[derive(
  Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default,
)]
pub struct LoginUser {
  pub email: String,
  pub password: String,
}

#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
#[derive(
  Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default,
)]
pub struct Session {
  pub session_key: String,
  pub user_id: i64,
}

impl From<HashMap<String, String>> for Session {
  fn from(value: HashMap<String, String>) -> Self {
    Self {
      session_key: value["session_key"].clone(),
      user_id: value["actix_identity.user_id"]
        .clone()
        .parse::<i64>()
        .unwrap(),
    }
  }
}

// TODO: rather rudimentary implementation rework consider macro use or smth else
// so one cannot forget to change this conversion when adding new fields
impl From<Session> for HashMap<String, String> {
  fn from(session: Session) -> Self {
    let mut map = HashMap::new();
    map.insert("session_key".to_string(), session.session_key);
    map.insert(
      "actix_identity.user_id".to_string(),
      session.user_id.to_string(),
    );
    map
  }
}
