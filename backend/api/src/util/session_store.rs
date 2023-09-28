use std::{collections::HashMap, num::ParseIntError};

use actix_session::storage::{
  LoadError, SaveError, SessionKey, SessionStore, UpdateError,
};
use actix_web::cookie::time::Duration;
use rand::{distributions::Alphanumeric, rngs::OsRng, Rng};
use shared::models::Session;

pub type SessionState = HashMap<String, String>;

#[derive(Clone)]
pub struct PostgresSessionStore {
  pool: sqlx::PgPool,
}

impl PostgresSessionStore {
  pub fn new(pool: sqlx::PgPool) -> Self {
    Self { pool }
  }
}

async fn user_id_str_as_i64(user_id: &str) -> Result<i64, ParseIntError> {
  let chars_to_trim: &[char] = &['"', '\\'];
  let user_id: &str = user_id.trim_matches(chars_to_trim);
  user_id.parse::<i64>()
}

#[async_trait::async_trait(?Send)]
impl SessionStore for PostgresSessionStore {
  async fn load(
    &self,
    session_key: &SessionKey,
  ) -> Result<Option<SessionState>, LoadError> {
    match sqlx::query_as::<_, Session>(
      r#"SELECT * FROM sessions WHERE session_key = $1"#,
    )
    .bind(session_key.as_ref())
    .fetch_one(&self.pool)
    .await
    {
      Ok(session) => Ok(Some({
        let session = <HashMap<String, String>>::from(session);
        session
      })),
      Err(e) => Err(LoadError::Deserialization(e.into())),
    }
  }

  async fn save(
    &self,
    session_state: SessionState,
    _ttl: &Duration,
  ) -> Result<SessionKey, SaveError> {
    let session_key = generate_session_key().await;

    let user_id = session_state["actix_identity.user_id"].clone();
    let user_id = user_id_str_as_i64(&user_id)
      .await
      .map_err(|e| SaveError::Serialization(e.into()))?;
    match sqlx::query_as::<_, Session>(
      r#"INSERT INTO sessions (session_key, user_id)
                VALUES ($1, $2)
                RETURNING *
                "#,
    )
    .bind(session_key.as_ref())
    .bind(&user_id) // 1
    .fetch_one(&self.pool) // 6
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
    session_key: SessionKey,
    session_state: SessionState,
    _ttl: &Duration,
  ) -> Result<SessionKey, UpdateError> {
    let user_id = &session_state["actix_identity.user_id"];
    let user_id = user_id_str_as_i64(user_id)
      .await
      .map_err(|e| UpdateError::Serialization(e.into()))?;

    match sqlx::query_as::<_, Session>(
      r#"
            UPDATE sessions
            SET session_key = $1, user_id = $2
            WHERE session_key = $1
            RETURNING *
            "#,
    )
    .bind(session_key.as_ref())
    .bind(&user_id) // 1
    .fetch_one(&self.pool) // 6
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
    session_key: &SessionKey,
  ) -> Result<(), anyhow::Error> {
    match sqlx::query_as::<_, Session>(
      // how do I delete row where session_key = $1
      r#"
          DELETE FROM sessions
          WHERE session_key = $1
          "#,
    )
    .bind(session_key.as_ref()) // 1
    .fetch_optional(&self.pool)
    .await
    {
      Ok(_) => Ok(()),
      Err(e) => Err(anyhow::anyhow!("Error deleting session: {:?}", e)),
    }
  }
}
/// sample 256 bit of data from alphanumeric distribution
async fn generate_session_key() -> SessionKey {
  let value = std::iter::repeat(())
    .map(|()| OsRng.sample(Alphanumeric))
    .take(64)
    .collect::<Vec<_>>();

  // These unwraps will never panic because pre-conditions are always verified
  // (i.e. length and character set)
  String::from_utf8(value).unwrap().try_into().unwrap()
}
