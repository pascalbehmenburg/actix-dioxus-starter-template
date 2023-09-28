// use actix_session::storage::SessionKey;
// use shared::models::Session;

// use crate::util::session_store::SessionState;
// use anyhow::Result;

// #[async_trait::async_trait]
// pub trait SessionRepository: Send + Sync + 'static {
//   async fn load(&self) -> Result<Session>;
//   async fn save(&self, session_state: SessionState) -> Result<SessionKey>;
//   async fn update(
//     &self,
//     session_key: SessionKey,
//     session_state: SessionState,
//   ) -> Result<SessionKey>;
//   async fn delete(&self, session_key: SessionKey) -> Result<()>;
// }

// pub struct PostgresSessionRepository {
//   pool: sqlx::PgPool,
// }

// impl PostgresSessionRepository {
//   pub fn new(pool: sqlx::PgPool) -> Self {
//     Self { pool }
//   }
// }

// #[async_trait::async_trait]
// impl SessionRepository for PostgresSessionRepository {
//   async fn load(&self, session_key: &SessionKey) -> Result<Session> {}
//   async fn save(&self, session_state: SessionState) -> Result<SessionKey>;
//   async fn update(
//     &self,
//     session_key: SessionKey,
//     session_state: SessionState,
//   ) -> Result<SessionKey>;
//   async fn delete(&self, session_key: SessionKey) -> Result<()>;
// }
