use shared::models::{CreateTodo, Todo};

mod postgres_todo_repository;

pub type TodoError = String;
pub type TodoResult<T> = Result<T, TodoError>;

pub use postgres_todo_repository::PostgresTodoRepository;

#[async_trait::async_trait]
pub trait TodoRepository: Send + Sync + 'static {
    async fn get_todos(&self) -> TodoResult<Vec<Todo>>;
    async fn get_todo(&self, id: &i32) -> TodoResult<Todo>;
    async fn create_todo(&self, id: &CreateTodo) -> TodoResult<Todo>;
    async fn update_todo(&self, id: &Todo) -> TodoResult<Todo>;
    async fn delete_todo(&self, id: &i32) -> TodoResult<i32>;
}
