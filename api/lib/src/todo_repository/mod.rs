use shared::models::{CreateTodo, UpdateTodo};

mod postgres_todo_repository;

pub use postgres_todo_repository::PostgresTodoRepository;

use crate::response::ApiResponse;

#[async_trait::async_trait]
pub trait TodoRepository: Send + Sync + 'static {
  async fn get_todos(&self) -> ApiResponse;
  async fn get_todo(&self, id: &i64) -> ApiResponse;
  async fn create_todo(&self, create_todo: &CreateTodo) -> ApiResponse;
  async fn update_todo(&self, update_todo: &UpdateTodo) -> ApiResponse;
  async fn delete_todo(&self, id: &i64) -> ApiResponse;
}
