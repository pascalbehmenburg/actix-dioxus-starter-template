use shared::models::{CreateTodo, Todo};

use super::{TodoRepository, TodoResult};

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
    async fn get_todos(&self) -> TodoResult<Vec<Todo>> {
        sqlx::query_as::<_, Todo>(
            r#"
            SELECT id, title, description, is_done, created_at, updated_at
            FROM todos
            ORDER BY id
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn get_todo(&self, todo_id: &i32) -> TodoResult<Todo> {
        sqlx::query_as::<_, Todo>(
            r#"
                SELECT id, title, description, is_done, created_at, updated_at
                FROM todos
                WHERE id = $1
                "#,
        )
        .bind(todo_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn create_todo(&self, create_todo: &CreateTodo) -> TodoResult<Todo> {
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
        .await
        .map_err(|e| e.to_string())
    }

    async fn update_todo(&self, todo: &Todo) -> TodoResult<Todo> {
        sqlx::query_as::<_, Todo>(
            r#"
                UPDATE todos
                SET title = $2, description = $3, is_done = $4
                WHERE id = $1
                RETURNING id, title, director, year, poster, created_at, updated_at
                "#,
        )
        .bind(todo.id)
        .bind(&todo.title)
        .bind(&todo.description)
        .bind(todo.is_done)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn delete_todo(&self, todo_id: &i32) -> TodoResult<i32> {
        sqlx::query_scalar::<_, i32>(
            r#"
                DELETE FROM todos
                WHERE id = $1
                RETURNING id
                "#,
        )
        .bind(todo_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }
}
