use shared::models::{CreateTodo, Todo, UpdateTodo};

use super::error::{Operation, RepositoryError};
use crate::util::response::JsonResponse;

const RELATION: &str = "Todo";

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
      sqlx::Error::RowNotFound => RepositoryError::NotFound {
        relation_name: RELATION.to_string(),
      },
      _ => RepositoryError::Other(e.into()),
    })
    .into()
  }

  async fn get_todo(
    &self,
    todo_id: &i64,
    session_user_id: &i64,
  ) -> JsonResponse {
    let todo: Todo = sqlx::query_as::<_, Todo>(
      r#"
      SELECT *
      FROM todos
      WHERE id = $1
      "#,
    )
    .bind::<&i64>(todo_id)
    .bind::<&i64>(session_user_id)
    .fetch_one(&self.pool)
    .await
    .map_err(|e| match e {
      sqlx::Error::RowNotFound => RepositoryError::NotFound {
        relation_name: RELATION.to_string(),
      },
      e => RepositoryError::Other(e.into()),
    })?;

    // check if session user that made the request
    // is the actual owner of the todo that was requested
    if todo.owner == *session_user_id {
      Ok(todo)
    } else {
      Err(RepositoryError::Forbidden {
        operation: Operation::Receive,
        relation_name: RELATION.to_string(),
      })
    }
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
    .map_err(Into::into)
    .map_err(RepositoryError::Other)
    .into()
  }

  async fn update_todo(
    &self,
    update_todo: &UpdateTodo,
    session_user_id: &i64,
  ) -> JsonResponse {
    // TODO
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
      sqlx::Error::RowNotFound => RepositoryError::Forbidden {
        operation: Operation::Update,
        relation_name: RELATION.to_string(),
      },
      e => RepositoryError::Other(e.into()),
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
      sqlx::Error::RowNotFound => RepositoryError::Forbidden {
        operation: Operation::Delete,
        relation_name: RELATION.to_string(),
      },
      _ => RepositoryError::Other(e.into()),
    })
    .into()
  }
}
