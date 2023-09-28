use shared::models::{CreateUser, UpdateUser, User};

use crate::util::response::JsonResponse;

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync + 'static {
  async fn get_users(&self) -> JsonResponse;
  async fn get_user_by_id(&self, id: &i64) -> JsonResponse;
  async fn get_user_by_email(&self, email: &str) -> JsonResponse;
  async fn create_user(&self, create_user: &CreateUser) -> JsonResponse;
  async fn update_user(&self, update_user: &UpdateUser) -> JsonResponse;
  async fn delete_user(&self, id: &i64) -> JsonResponse;
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
  async fn get_users(&self) -> JsonResponse {
    sqlx::query_as::<_, User>(
            "SELECT id, name, email, password, created_at, updated_at FROM users ORDER BY id",
        )
        .fetch_all(&self.pool)
        .await
        .into()
  }

  async fn get_user_by_email(&self, email: &str) -> JsonResponse {
    sqlx::query_as::<_, User>(
      r#"
                SELECT id, name, email, password, created_at, updated_at
                FROM users
                WHERE email = $1
                "#,
    )
    .bind(email)
    .fetch_one(&self.pool)
    .await
    .into()
  }

  async fn get_user_by_id(&self, user_id: &i64) -> JsonResponse {
    sqlx::query_as::<_, User>(
      r#"
                SELECT id, name, email, password, created_at, updated_at
                FROM users
                WHERE id = $1
                "#,
    )
    .bind(user_id)
    .fetch_one(&self.pool)
    .await
    .into()
  }

  async fn create_user(&self, create_user: &CreateUser) -> JsonResponse {
    sqlx::query_as::<_, User>(
      r#"
                INSERT INTO users (name, email, password)
                VALUES ($1, $2, $3)
                RETURNING id, name, email, password, created_at, updated_at
                "#,
    )
    .bind(&create_user.name)
    .bind(&create_user.email)
    .bind(&create_user.password)
    .fetch_one(&self.pool)
    .await
    .into()
  }

  async fn update_user(&self, update_user: &UpdateUser) -> JsonResponse {
    sqlx::query_as::<_, User>(
      r#"
                UPDATE users
                SET name = $2, email = $3, password = $4, updated_at = now()
                WHERE id = $1
                RETURNING id, name, email, password, created_at, updated_at
                "#,
    )
    .bind(update_user.id)
    .bind(&update_user.name)
    .bind(&update_user.email)
    .bind(&update_user.password)
    .fetch_one(&self.pool)
    .await
    .into()
  }

  async fn delete_user(&self, user_id: &i64) -> JsonResponse {
    sqlx::query_scalar::<_, i64>(
      r#"
                DELETE FROM users
                WHERE id = $1
                RETURNING id
                "#,
    )
    .bind(user_id)
    .fetch_one(&self.pool)
    .await
    .into()
  }
}
