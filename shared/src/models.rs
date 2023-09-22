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
  pub created_at: chrono::DateTime<chrono::Utc>,
  pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
#[derive(
  Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default,
)]
pub struct CreateTodo {
  pub title: String,
  pub description: String,
}

// TODO: This should have Option<> fields so that the client may decide which fields to update
#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
#[derive(
  Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default,
)]
pub struct UpdateTodo {
  pub id: i64,
  pub title: String,
  pub description: String,
  pub is_done: bool,
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
  pub salt: String,
  pub created_at: chrono::DateTime<chrono::Utc>,
  pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
#[derive(
  Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default,
)]
pub struct CreateUser {
  pub name: String,
  pub email: String,
  pub password: String,
  pub salt: String,
}

#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
#[derive(
  Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default,
)]
pub struct UpdateUser {
  pub id: i64,
  pub name: String,
  pub email: String,
  pub password: String,
  pub salt: String,
}

#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
#[derive(
  Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default,
)]
pub struct Session {
  pub id: i64,
  pub session_key: String,
  pub device: String,
  pub created_at: chrono::DateTime<chrono::Utc>,
  pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<Session> for HashMap<String, String> {
  fn from(value: Session) -> Self {
    let mut map = HashMap::new();
    map.insert("id".to_string(), value.id.to_string());
    map.insert("session_key".to_string(), value.session_key.to_string());
    map.insert("device".to_string(), value.device);
    map.insert("created_at".to_string(), value.created_at.to_string());
    map.insert("updated_at".to_string(), value.updated_at.to_string());
    map
  }
}
