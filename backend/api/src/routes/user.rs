use actix_http::StatusCode;
use actix_identity::Identity;
use actix_session::Session;
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
  repository::UserRepository,
  util::{Error, Response},
};

pub fn service<R: UserRepository>(cfg: &mut ServiceConfig) {
  cfg.service(
    web::scope("/v1/users")
      // get all
      .route("", web::get().to(get_all::<R>))
      // get by id
      .route("/{user_id}", web::get().to(get::<R>))
      // new
      .route("/register", web::post().to(post::<R>))
      // update
      .route("", web::put().to(put::<R>))
      // delete
      .route("/{user_id}", web::delete().to(delete::<R>))
      // login
      .route("/login", web::post().to(login::<R>)),
  );
}

async fn login<R: UserRepository>(
  request: HttpRequest,
  login_user: web::Json<LoginUser>,
  repo: web::Data<R>,
  session: Session,
) -> Response {
  let user = repo.get_user_by_email(&login_user.email).await?.0;

  let user_agent = match request.headers().get("User-Agent") {
    Some(user_agent) => user_agent.to_str().unwrap_or("Default"),
    None => "Default",
  };

  session
    .insert("device", user_agent)
    .map_err(|e| Error::ActixWebServerError(e.into()))?;

  let user: User = serde_json::from_value(user)?;

  // get the password hash from the database
  let argon2 = Argon2::default();
  let parsed_hash = PasswordHash::new(&user.password)?;

  // verify the password hash from server with the one from the request
  if argon2
    .verify_password(login_user.password.as_bytes(), &parsed_hash)
    .is_ok()
  {
    Identity::login(&request.extensions(), user.id.to_string())
      .map_err(|e| Error::ActixWebServerError(e.into()))?;
    serde_json::to_value(user.id).into()
  } else {
    Response(Err(crate::util::Error::CustomHTTPResponse(
      StatusCode::UNAUTHORIZED,
      "Invalid password.".to_string(),
    )))
  }
}

async fn get_all<R: UserRepository>(repo: web::Data<R>) -> Response {
  repo.get_users().await
}

async fn get<R: UserRepository>(
  user_id: web::Path<i64>,
  repo: web::Data<R>,
) -> Response {
  repo.get_user_by_id(&user_id).await
}

async fn post<R: UserRepository>(
  create_user: web::Json<CreateUser>,
  repo: web::Data<R>,
) -> Response {
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
) -> Response {
  repo.update_user(&update_user).await
}

async fn delete<R: UserRepository>(
  user_id: web::Path<i64>,
  repo: web::Data<R>,
) -> Response {
  repo.delete_user(&user_id).await
}
