use shared::models::{CreateUser, UpdateUser};

mod postgres_user_repository;

pub use postgres_user_repository::PostgresUserRepository;

use crate::response::ApiResponse;
#[async_trait::async_trait]
pub trait UserRepository: Send + Sync + 'static {
  async fn get_users(&self) -> ApiResponse;
  async fn get_user_by_id(&self, id: &i64) -> ApiResponse;
  async fn get_user_by_email(&self, email: &str) -> ApiResponse;
  async fn create_user(&self, create_user: &CreateUser) -> ApiResponse;
  async fn update_user(&self, update_user: &UpdateUser) -> ApiResponse;
  async fn delete_user(&self, id: &i64) -> ApiResponse;
}
