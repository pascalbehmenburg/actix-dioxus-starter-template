use actix_http::StatusCode;
use actix_identity::Identity;
use actix_web::{
  web::{self, ServiceConfig},
  HttpMessage, HttpRequest,
};
use shared::models::{CreateUser, LoginUser, UpdateUser, User};

use argon2::{
  password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
  Argon2, PasswordHash, PasswordVerifier,
};

use crate::{
  repository::user::UserRepository,
  util::{error::Error, response::JsonResponse},
};

use super::error::UserError;

pub fn service<R: UserRepository>(cfg: &mut ServiceConfig) {
  cfg.service(
    web::scope("/v1/users")
      // get user by identity id
      .route("", web::get().to(get::<R>))
      // register - creates user (redirect needs to be done in frontend)
      .route("/register", web::post().to(post::<R>))
      // update user associated with sent identity id
      .route("", web::put().to(put::<R>))
      // delete's user associated with sent identity id
      .route("", web::delete().to(delete::<R>))
      // login -  creates session and sets identity id
      .route("/login", web::post().to(login::<R>)),
  );
}

async fn login<R: UserRepository>(
  request: HttpRequest,
  login_user: web::Json<LoginUser>,
  repo: web::Data<R>,
) -> JsonResponse {
  let user_val = repo
    .get_user_by_email(&login_user.email)
    .await
    .0
    .map_err(|_| UserError::InvalidEmailOrPassword)?;

  let user = serde_json::from_value::<User>(user_val.clone())?;

  let argon2 = Argon2::default();
  let parsed_hash = PasswordHash::new(&user.password)?;

  if argon2
    .verify_password(login_user.password.as_bytes(), &parsed_hash)
    .is_ok()
  {
    Identity::login(&request.extensions(), user.id.to_string())
      .map_err(Into::into)
      .map_err(Error::Other)?;

    JsonResponse(Ok(user_val))
  } else {
    JsonResponse(Err(Error::External(
      StatusCode::UNAUTHORIZED,
      "Invalid email or password.".to_string(),
    )))
  }
}

async fn get<R: UserRepository>(
  repo: web::Data<R>,
  user: Identity,
) -> JsonResponse {
  let session_user_id = super::get_identity_id(user).await?;

  repo.get_session_user(&session_user_id).await
}

async fn post<R: UserRepository>(
  create_user: web::Json<CreateUser>,
  repo: web::Data<R>,
) -> JsonResponse {
  let argon2 = Argon2::default();
  let password_hash = argon2
    .hash_password(
      create_user.password.as_bytes(),
      &SaltString::generate(&mut OsRng),
    )?
    .to_string();

  let new_user = CreateUser {
    password: password_hash,
    ..create_user.into_inner()
  };

  repo.create_user(&new_user).await
}

async fn put<R: UserRepository>(
  update_user: web::Json<UpdateUser>,
  repo: web::Data<R>,
  user: Identity,
) -> JsonResponse {
  let session_user_id = super::get_identity_id(user).await?;

  repo.update_user(&update_user, &session_user_id).await
}

async fn delete<R: UserRepository>(
  repo: web::Data<R>,
  user: Identity,
) -> JsonResponse {
  let session_user_id = super::get_identity_id(user).await?;
  repo.delete_user(&session_user_id).await
}
