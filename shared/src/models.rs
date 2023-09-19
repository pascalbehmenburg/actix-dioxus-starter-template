use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub is_done: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct CreateTodo {
    pub title: String,
    pub description: String,
}

// TODO: This should have Option<> fields so that the client may decide which fields to update
#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct UpdateTodo {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub is_done: bool,
}

#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub salt: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct UpdateUser {
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}
