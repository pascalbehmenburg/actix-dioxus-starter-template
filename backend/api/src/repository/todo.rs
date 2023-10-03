use actix_http::StatusCode;
use shared::models::{CreateTodo, Todo, UpdateTodo};

use crate::util::{error::Error, response::JsonResponse};

#[async_trait::async_trait]
pub trait TodoRepository: Send + Sync + 'static {
  async fn get_todos(&self, session_user_id: &i64) -> JsonResponse;

  async fn get_todo(
    &self,
    todo_id: &i64,
    session_user_id: &i64,
  ) -> JsonResponse;

  async fn create_todo(
    &self,
    create_todo: &CreateTodo,
    session_user_id: &i64,
  ) -> JsonResponse;

  async fn update_todo(
    &self,
    update_todo: &UpdateTodo,
    session_user_id: &i64,
  ) -> JsonResponse;

  async fn delete_todo(&self, id: &i64, session_user_id: &i64) -> JsonResponse;
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
  async fn get_todos(&self, session_user_id: &i64) -> JsonResponse {
    sqlx::query_as::<_, Todo>(
      r#"
      SELECT *
      FROM todos
      WHERE owner = $1
      ORDER BY id"#,
    )
    .bind::<&i64>(session_user_id)
    .fetch_all(&self.pool)
    .await
    .map_err(|e| match e {
      sqlx::Error::RowNotFound => Error::CustomHTTPResponse(
        StatusCode::FORBIDDEN,
        "It seems like you created no todos yet, try to create one."
          .to_string(),
      ),
      _ => e.into(),
    })
    .into()
  }

  async fn get_todo(
    &self,
    todo_id: &i64,
    session_user_id: &i64,
  ) -> JsonResponse {
    sqlx::query_as::<_, Todo>(
      r#"
      SELECT *
      FROM todos
      WHERE id = $1 and owner = $2
      "#,
    )
    .bind::<&i64>(todo_id)
    .bind::<&i64>(session_user_id)
    .fetch_one(&self.pool)
    .await
    .map_err(|e| match e {
      sqlx::Error::RowNotFound => Error::CustomHTTPResponse(
        StatusCode::FORBIDDEN,
        "You are not the owner of the todo you are trying to receive."
          .to_string(),
      ),
      _ => e.into(),
    })
    .into()
  }

  async fn create_todo(
    &self,
    create_todo: &CreateTodo,
    session_user_id: &i64,
  ) -> JsonResponse {
    sqlx::query_as::<_, Todo>(
      r#"
      INSERT
      INTO todos (title, description, owner)
      VALUES ($1, $2, $3)
      RETURNING *
      "#,
    )
    .bind::<&str>(&create_todo.title)
    .bind::<&str>(&create_todo.description)
    .bind::<&i64>(session_user_id)
    .fetch_one(&self.pool)
    .await
    .into()
  }

  async fn update_todo(
    &self,
    update_todo: &UpdateTodo,
    session_user_id: &i64,
  ) -> JsonResponse {
    sqlx::query_as::<_, Todo>(
      r#"
      UPDATE todos
      SET 
        title = COALESCE($1, title),
        description = COALESCE($2, description),
        is_done = COALESCE($3, is_done),
        updated_at = NOW()
      WHERE id = $4 and owner = $5
      RETURNING *
      "#,
    )
    .bind::<&Option<String>>(&update_todo.title)
    .bind::<&Option<String>>(&update_todo.description)
    .bind::<&Option<bool>>(&update_todo.is_done)
    .bind::<&i64>(&update_todo.id)
    .bind::<&i64>(session_user_id)
    .fetch_one(&self.pool)
    .await
    .map_err(|e| match e {
      sqlx::Error::RowNotFound => Error::CustomHTTPResponse(
        StatusCode::FORBIDDEN,
        "You are not the owner of the todo you are trying to modify."
          .to_string(),
      ),
      _ => e.into(),
    })
    .into()
  }

  async fn delete_todo(
    &self,
    todo_id: &i64,
    session_user_id: &i64,
  ) -> JsonResponse {
    sqlx::query(
      r#"
      DELETE
      FROM todos
      WHERE id = $1 and owner = $2
      "#,
    )
    .bind::<&i64>(todo_id)
    .bind::<&i64>(session_user_id)
    .execute(&self.pool)
    .await
    .map(|_| ())
    .map_err(|e| match e {
      sqlx::Error::RowNotFound => Error::CustomHTTPResponse(
        StatusCode::FORBIDDEN,
        "You are not the owner of the todo you tried to delete.".to_string(),
      ),
      _ => e.into(),
    })
    .into()
  }
}
