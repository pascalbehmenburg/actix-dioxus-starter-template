use actix_identity::Identity;

use crate::util::error::Error;

pub mod error;
pub mod health;
pub mod todo;
pub mod user;

// helper function for routes to use when getting the identity id as an i64 since it is saved as such in the repository that is in use right now
async fn get_identity_id(identity: Identity) -> Result<i64, Error> {
  let identity_id = identity.id().map_err(|e| Error::Other(e.into()))?;

  identity_id
    .parse::<i64>()
    .map_err(|e| Error::Other(e.into()))
}
