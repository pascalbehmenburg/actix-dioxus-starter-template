use actix_http::StatusCode;
use shared::models::{Todo, UpdateTodo};

use crate::util::{Error, Response};

#[async_trait::async_trait]
pub trait TodoRepository: Send + Sync + 'static {
  async fn get_todos(&self) -> Response;
  async fn get_todo(&self, id: &i64) -> Response;
  async fn create_todo(&self, create_todo: &Todo) -> Response;
  async fn update_todo(&self, update_todo: &UpdateTodo) -> Response;
  async fn delete_todo(&self, id: &i64) -> Response;
}

pub struct PostgresTodoRepository {
  pool: sqlx::PgPool,
}

impl PostgresTodoRepository {
  pub fn new(pool: sqlx::PgPool) -> Self {
    Self { pool }
  }
}

#[async_trait::async_trait]
impl TodoRepository for PostgresTodoRepository {
  async fn get_todos(&self) -> Response {
    sqlx::query_as::<_,Todo>(
            "SELECT id, title, description, is_done, owner, created_at, updated_at FROM todos ORDER BY id",
        )
        .fetch_all(&self.pool)
        .await
        .into()
  }

  async fn get_todo(&self, todo_id: &i64) -> Response {
    sqlx::query_as::<_, Todo>(
      r#"
                SELECT id, title, description, is_done, owner, created_at, updated_at
                FROM todos
                WHERE id = $1
                "#,
    )
    .bind(todo_id)
    .fetch_one(&self.pool)
    .await
    .into()
  }

  async fn create_todo(&self, todo: &Todo) -> Response {
    sqlx::query_as::<_, Todo>(
            r#"
                INSERT INTO todos (title, description)
                VALUES ($1, $2)
                RETURNING id, title, description, owner, is_done, created_at, updated_at
                "#,
        )
        .bind(&todo.title)
        .bind(&todo.description)
        .fetch_one(&self.pool)
        .await
        .into()
  }

  async fn update_todo(&self, update_todo: &UpdateTodo) -> Response {
    sqlx::query_as::<_, Todo>(
            r#"
                UPDATE todos
                SET title = $2, description = $3, is_done = $4, updated_at = now()
                WHERE id = $1, owner = $5
                RETURNING id, title, description, owner, is_done, created_at, updated_at
                "#,
        )
        .bind(update_todo.id)
        .bind(&update_todo.title)
        .bind(&update_todo.description)
        .bind(&update_todo.is_done)
        .bind(&update_todo.owner)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
          sqlx::Error::RowNotFound => Error::CustomHTTPResponse(StatusCode::FORBIDDEN, "You are not authorized to edit this todo or the todo does not exist.".to_string()),
          _ => e.into()
        }).into()
  }

  async fn delete_todo(&self, todo_id: &i64) -> Response {
    sqlx::query_scalar::<_, i64>(
      r#"
                DELETE FROM todos
                WHERE id = $1
                RETURNING id
                "#,
    )
    .bind(todo_id)
    .fetch_one(&self.pool)
    .await
    .into()
  }
}
