use std::collections::HashMap;

use actix_session::storage::{
  LoadError, SaveError, SessionKey, SessionStore, UpdateError,
};
use actix_web::cookie::time::Duration;
use shared::models::Session;

pub(crate) type SessionState = HashMap<String, String>;

#[derive(Clone)]
pub struct PostgresSessionStore {
  pool: sqlx::PgPool,
}

impl PostgresSessionStore {
  pub fn new(pool: sqlx::PgPool) -> Self {
    Self { pool }
  }
}

#[async_trait::async_trait(?Send)]
impl SessionStore for PostgresSessionStore {
  async fn load(
    &self,
    session_key: &SessionKey,
  ) -> Result<Option<SessionState>, LoadError> {
    match sqlx::query_as::<_, Session>(
      "SELECT * FROM sessions WHERE session_key = $1",
    )
    .bind(session_key.as_ref())
    .fetch_optional(&self.pool)
    .await
    {
      Ok(session) => Ok(session.map(From::from)),
      Err(e) => Err(LoadError::Other(e.into())),
    }
  }

  async fn save(
    &self,
    session_state: SessionState,
    _ttl: &Duration,
  ) -> Result<SessionKey, SaveError> {
    match sqlx::query_as::<_, Session>(
      r#"
                INSERT INTO sessions (session_key, device)
                VALUES ($1, $2)
                RETURNING id, session_key, device, created_at, updated_at
                "#,
    )
    .bind(&session_state["session_key"])
    .bind(&session_state["device"])
    .fetch_one(&self.pool)
    .await
    {
      Ok(session) => Ok(
        session
          .session_key
          .try_into()
          .map_err(Into::into)
          .map_err(SaveError::Other)?,
      ),
      Err(e) => Err(SaveError::Serialization(e.into())),
    }
  }

  async fn update(
    &self,
    _session_key: SessionKey,
    session_state: SessionState,
    _ttl: &Duration,
  ) -> Result<SessionKey, UpdateError> {
    match sqlx::query_as::<_, Session>(
      r#"
                UPDATE sessions
                SET session_key = $1, device = $2, updated_at = NOW()
                WHERE session_key = $1
                RETURNING id, session_key, device, created_at, updated_at
                "#,
    )
    .bind(&session_state["session_key"])
    .bind(&session_state["device"])
    .fetch_one(&self.pool)
    .await
    {
      Ok(session) => Ok(
        session
          .session_key
          .try_into()
          .map_err(Into::into)
          .map_err(UpdateError::Other)?,
      ),
      Err(e) => Err(UpdateError::Serialization(e.into())),
    }
  }

  async fn update_ttl(
    &self,
    _session_key: &SessionKey,
    _ttl: &Duration,
  ) -> Result<(), anyhow::Error> {
    Ok(())
  }
  // TODO: give the user the option to end a session via logout / list of devices that are logged in
  async fn delete(
    &self,
    _session_key: &SessionKey,
  ) -> Result<(), anyhow::Error> {
    Ok(())
  }
}
