use shared::models::{CreateTodo, Todo, UpdateTodo};

use crate::response::ApiResponse;

use super::TodoRepository;

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
  async fn get_todos(&self) -> ApiResponse {
    sqlx::query_as::<_, Todo>(
            "SELECT id, title, description, is_done, created_at, updated_at FROM todos ORDER BY id",
        )
        .fetch_all(&self.pool)
        .await?
        .into()
  }

  async fn get_todo(&self, todo_id: &i64) -> ApiResponse {
    sqlx::query_as::<_, Todo>(
      r#"
                SELECT id, title, description, is_done, created_at, updated_at
                FROM todos
                WHERE id = $1
                "#,
    )
    .bind(todo_id)
    .fetch_one(&self.pool)
    .await?
    .into()
  }

  async fn create_todo(&self, create_todo: &CreateTodo) -> ApiResponse {
    sqlx::query_as::<_, Todo>(
            r#"
                INSERT INTO todos (title, description)
                VALUES ($1, $2)
                RETURNING id, title, description, is_done, created_at, updated_at
                "#,
        )
        .bind(&create_todo.title)
        .bind(&create_todo.description)
        .fetch_one(&self.pool)
        .await?
        .into()
  }

  async fn update_todo(&self, update_todo: &UpdateTodo) -> ApiResponse {
    sqlx::query_as::<_, Todo>(
            r#"
                UPDATE todos
                SET title = $2, description = $3, is_done = $4, updated_at = now()
                WHERE id = $1
                RETURNING id, title, description, is_done, created_at, updated_at
                "#,
        )
        .bind(update_todo.id)
        .bind(&update_todo.title)
        .bind(&update_todo.description)
        .bind(update_todo.is_done)
        .fetch_one(&self.pool)
        .await?
        .into()
  }

  async fn delete_todo(&self, todo_id: &i64) -> ApiResponse {
    sqlx::query_scalar::<_, i64>(
      r#"
                DELETE FROM todos
                WHERE id = $1
                RETURNING id
                "#,
    )
    .bind(todo_id)
    .fetch_one(&self.pool)
    .await?
    .into()
  }
}
