use shared::models::{CreateUser, UpdateUser, User};

mod postgres_user_repository;

pub type UserError = String;
pub type UserResult<T> = Result<T, UserError>;

pub use postgres_user_repository::PostgresUserRepository;

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync + 'static {
    async fn get_users(&self) -> UserResult<Vec<User>>;
    async fn get_user(&self, id: &i64) -> UserResult<User>;
    async fn create_user(&self, create_user: &CreateUser) -> UserResult<User>;
    async fn update_user(&self, update_user: &UpdateUser) -> UserResult<User>;
    async fn delete_user(&self, id: &i64) -> UserResult<i64>;
}
