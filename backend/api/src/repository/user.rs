use shared::models::{CreateUser, UpdateUser, User};

use crate::util::response::JsonResponse;

use super::error::{Operation, RepositoryError};

const RELATION: &str = "User";

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync + 'static {
  async fn get_session_user(&self, session_user_id: &i64) -> JsonResponse;

  // SECURITY
  // if exposed in an endpoint doxing a user is possible (check if email is registered)
  async fn get_user_by_email(&self, email: &str) -> JsonResponse;

  async fn create_user(&self, create_user: &CreateUser) -> JsonResponse;

  async fn update_user(
    &self,
    update_user: &UpdateUser,
    session_user_id: &i64,
  ) -> JsonResponse;

  async fn delete_user(&self, session_user_id: &i64) -> JsonResponse;
}

pub struct PostgresUserRepository {
  pool: sqlx::PgPool,
}

impl PostgresUserRepository {
  pub fn new(pool: sqlx::PgPool) -> Self {
    Self { pool }
  }
}

#[async_trait::async_trait]
impl UserRepository for PostgresUserRepository {
  async fn get_user_by_email(&self, email: &str) -> JsonResponse {
    sqlx::query_as::<_, User>(
      r#"
      SELECT *
      FROM users
      WHERE email = $1
      "#,
    )
    .bind::<&str>(email)
    .fetch_one(&self.pool)
    .await
    .map_err(|e| match e {
      sqlx::Error::RowNotFound => RepositoryError::NotFound {
        relation_name: RELATION.to_string(),
      },
      _ => RepositoryError::Other(e.into()),
    })
    .into()
  }

  async fn get_session_user(&self, session_user_id: &i64) -> JsonResponse {
    // this should literally not be possible to fail as long as the Database is online
    sqlx::query_as::<_, User>(
      r#"
      SELECT *
      FROM users
      WHERE id = $1
      "#,
    )
    .bind::<&i64>(session_user_id)
    .fetch_one(&self.pool)
    .await
    .map_err(Into::into)
    .map_err(RepositoryError::Other)
    .into()
  }

  async fn create_user(&self, create_user: &CreateUser) -> JsonResponse {
    sqlx::query_as::<_, User>(
      r#"
      INSERT
      INTO users (name, email, password)
      VALUES ($1, $2, $3)
      RETURNING *
      "#,
    )
    .bind::<&str>(&create_user.name)
    .bind::<&str>(&create_user.email)
    .bind::<&str>(&create_user.password)
    .fetch_one(&self.pool)
    .await
    .map_err(Into::into)
    .map_err(RepositoryError::Other)
    .into()
  }

  // TODO apply same principle as in todo with optional type fields
  async fn update_user(
    &self,
    update_user: &UpdateUser,
    session_user_id: &i64,
  ) -> JsonResponse {
    sqlx::query_as::<_, User>(
      r#"
      UPDATE users
      SET 
        name = COALESCE($1, name),
        email = COALESCE($2, email),
        password = COALESCE($3, password),
        updated_at = now()
      WHERE id = $4
      RETURNING *
      "#,
    )
    .bind::<&Option<String>>(&update_user.name)
    .bind::<&Option<String>>(&update_user.email)
    .bind::<&Option<String>>(&update_user.password)
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

  async fn delete_user(&self, session_user_id: &i64) -> JsonResponse {
    sqlx::query(
      r#"
      DELETE
      FROM users
      WHERE id = $1
      "#,
    )
    .bind::<&i64>(session_user_id)
    .execute(&self.pool)
    .await
    .map(|_| ())
    .map_err(|e| match e {
      sqlx::Error::RowNotFound => RepositoryError::NotFound {
        relation_name: RELATION.to_string(),
      },
      e => RepositoryError::Other(e.into()),
    })
    .into()
  }
}
