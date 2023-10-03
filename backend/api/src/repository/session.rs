use actix_session::storage::{
  LoadError, SaveError, SessionKey, SessionStore, UpdateError,
};
use actix_web::cookie::time::Duration;
use anyhow::Context;
use rand::{distributions::Alphanumeric, rngs::OsRng, Rng};
use sqlx::Row;
use std::collections::HashMap;

pub type SessionState = HashMap<String, String>;

#[async_trait::async_trait]
pub trait SessionRepository: Send + Sync + 'static {
  async fn db_load(
    &self,
    session_key: &SessionKey,
  ) -> Result<Option<serde_json::Value>, sqlx::Error>;

  async fn db_save(
    &self,
    session_key: &SessionKey,
    session_state: &serde_json::Value,
  ) -> Result<(), sqlx::Error>;

  async fn db_update(
    &self,
    session_key: &SessionKey,
    session_state: &serde_json::Value,
  ) -> Result<(), sqlx::Error>;

  async fn db_delete(
    &self,
    session_key: &SessionKey,
  ) -> Result<(), sqlx::Error>;

  async fn generate_session_key() -> SessionKey {
    let value = std::iter::repeat(())
      .map(|()| OsRng.sample(Alphanumeric))
      .take(64)
      .collect::<Vec<_>>();
    String::from_utf8(value).unwrap().try_into().unwrap()
  }
}

#[derive(Clone)]
pub struct PostgresSessionRepository {
  pool: sqlx::PgPool,
}

impl PostgresSessionRepository {
  pub fn new(pool: sqlx::PgPool) -> Self {
    Self { pool }
  }
}

#[async_trait::async_trait]
impl SessionRepository for PostgresSessionRepository {
  async fn db_load(
    &self,
    session_key: &SessionKey,
  ) -> Result<Option<serde_json::Value>, sqlx::Error> {
    let response = sqlx::query(r#"SELECT * FROM sessions WHERE key = $1"#)
      .bind::<&str>(session_key.as_ref())
      .fetch_optional(&self.pool)
      .await?;

    let session_state: Option<serde_json::Value> =
      response.map(|row| row.get("state"));

    Ok(session_state)
  }

  async fn db_save(
    &self,
    session_key: &SessionKey,
    session_state: &serde_json::Value,
  ) -> Result<(), sqlx::Error> {
    sqlx::query(
      r#"INSERT
      INTO sessions (key, state)
      VALUES ($1, $2)
      "#,
    )
    .bind::<&str>(session_key.as_ref())
    .bind::<&serde_json::Value>(session_state)
    .execute(&self.pool)
    .await
    .map(|_| ())
  }

  async fn db_update(
    &self,
    session_key: &SessionKey,
    session_state: &serde_json::Value,
  ) -> Result<(), sqlx::Error> {
    sqlx::query(
      r#"
      UPDATE sessions
      SET key = $1, state = $2
      WHERE key = $1
      "#,
    )
    .bind::<&str>(session_key.as_ref())
    .bind::<&serde_json::Value>(session_state)
    .execute(&self.pool)
    .await
    .map(|_| ())
  }

  async fn db_delete(
    &self,
    session_key: &SessionKey,
  ) -> Result<(), sqlx::Error> {
    sqlx::query(
      r#"
      DELETE
      FROM sessions
      WHERE key = $1
      "#,
    )
    .bind::<&str>(session_key.as_ref())
    .execute(&self.pool)
    .await
    .map(|_| ())
  }
}

#[async_trait::async_trait(?Send)]
impl SessionStore for PostgresSessionRepository {
  async fn load(
    &self,
    session_key: &SessionKey,
  ) -> Result<Option<SessionState>, LoadError> {
    self
      .db_load(session_key)
      .await
      .map_err(Into::into)
      .map_err(LoadError::Other)
      .and_then(|opt_value| {
        opt_value
          .map(|value| {
            serde_json::from_value::<SessionState>(value)
              .map_err(Into::into)
              .map_err(LoadError::Deserialization)
          })
          .transpose()
      })
  }

  async fn save(
    &self,
    session_state: SessionState,
    _ttl: &Duration,
  ) -> Result<SessionKey, SaveError> {
    let session_key: SessionKey = Self::generate_session_key().await;

    let session_state: serde_json::Value = serde_json::to_value(session_state)
      .map_err(Into::into)
      .map_err(SaveError::Serialization)?;

    self
      .db_save(&session_key, &session_state)
      .await
      .map_err(Into::into)
      .map_err(SaveError::Other)?;

    Ok(session_key)
  }

  async fn update(
    &self,
    session_key: SessionKey,
    session_state: SessionState,
    _ttl: &Duration,
  ) -> Result<SessionKey, UpdateError> {
    let session_state: serde_json::Value = serde_json::to_value(session_state)
      .map_err(Into::into)
      .map_err(UpdateError::Serialization)?;

    self
      .db_update(&session_key, &session_state)
      .await
      .map_err(Into::into)
      .map_err(UpdateError::Other)?;

    Ok(session_key)
  }

  async fn update_ttl(
    &self,
    _session_key: &SessionKey,
    _ttl: &Duration,
  ) -> Result<(), anyhow::Error> {
    Ok(())
  }

  async fn delete(
    &self,
    session_key: &SessionKey,
  ) -> Result<(), anyhow::Error> {
    self
      .db_delete(session_key)
      .await
      .map_err(anyhow::Error::from)
      .context(
        "Some psql error occurred when trying to delete session from db.",
      )
  }
}
