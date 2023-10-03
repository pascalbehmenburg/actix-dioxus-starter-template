use actix_http::StatusCode;
use shared::models::{CreateUser, UpdateUser, User};

use crate::util::{error::Error, response::JsonResponse};

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
    .into()
  }

  async fn get_session_user(&self, session_user_id: &i64) -> JsonResponse {
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
    .map_err(|e| match e {
      sqlx::Error::RowNotFound => Error::CustomHTTPResponse(
        StatusCode::FORBIDDEN,
        "You are not the owner of the user data you are trying to receive."
          .to_string(),
      ),
      _ => e.into(),
    })
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
        name = $1,
        email = $2,
        password = $3,
        updated_at = now()
      WHERE id = $4
      RETURNING *
      "#,
    )
    .bind::<&str>(&update_user.name)
    .bind::<&str>(&update_user.email)
    .bind::<&str>(&update_user.password)
    .bind::<&i64>(session_user_id)
    .fetch_one(&self.pool)
    .await
    .map_err(|e| match e {
      sqlx::Error::RowNotFound => Error::CustomHTTPResponse(
        StatusCode::FORBIDDEN,
        "You are not the owner of the user you are trying to modify."
          .to_string(),
      ),
      _ => e.into(),
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
      sqlx::Error::RowNotFound => Error::CustomHTTPResponse(
        StatusCode::FORBIDDEN,
        "You are not the owner of the user you tried to delete.".to_string(),
      ),
      _ => e.into(),
    })
    .into()
  }
}
