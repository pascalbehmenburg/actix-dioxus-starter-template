use shared::models::{CreateUser, UpdateUser, User};

use super::{UserRepository, UserResult};

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
    async fn get_users(&self) -> UserResult<Vec<User>> {
        sqlx::query_as::<_, User>(
            "SELECT id, name, email, password, salt, created_at, updated_at FROM users ORDER BY id",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn get_user(&self, user_id: &i64) -> UserResult<User> {
        sqlx::query_as::<_, User>(
            r#"
                SELECT id, name, email, password, salt, created_at, updated_at
                FROM users
                WHERE id = $1
                "#,
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn create_user(&self, create_user: &CreateUser) -> UserResult<User> {
        sqlx::query_as::<_, User>(
            r#"
                INSERT INTO users (name, email, password, salt)
                VALUES ($1, $2, $3, $4)
                RETURNING id, name, email, password, salt, created_at, updated_at
                "#,
        )
        .bind(&create_user.name)
        .bind(&create_user.email)
        .bind(&create_user.password)
        .bind(&create_user.salt)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn update_user(&self, update_user: &UpdateUser) -> UserResult<User> {
        sqlx::query_as::<_, User>(
            r#"
                UPDATE users
                SET name = $2, email = $3, password = $4, salt = $5
                WHERE id = $1
                RETURNING id, name, email, password, salt, created_at, updated_at
                "#,
        )
        .bind(&update_user.id)
        .bind(&update_user.name)
        .bind(&update_user.email)
        .bind(&update_user.password)
        .bind(&update_user.salt)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn delete_user(&self, user_id: &i64) -> UserResult<i64> {
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
        .map_err(|e| e.to_string())
    }
}
