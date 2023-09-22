use actix_identity::Identity;
use actix_web::{
  web::{self, ServiceConfig},
  HttpMessage, HttpRequest,
};
use serde_json::json;
use shared::models::{CreateUser, LoginUser, UpdateUser, User};

use argon2::{
  password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
  Argon2, PasswordHash, PasswordVerifier,
};

use crate::{
  error::{ApiError, ApiErrorKind},
  response::{ApiData, ApiResponse},
  user_repository::UserRepository,
};

pub fn service<R: UserRepository>(cfg: &mut ServiceConfig) {
  cfg.service(
    web::scope("/v1/users")
      // get all
      .route("", web::get().to(get_all::<R>))
      // get by id
      .route("/{user_id}", web::get().to(get::<R>))
      // new
      .route("", web::post().to(post::<R>))
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
) -> ApiResponse {
  let user = repo.get_user_by_email(&login_user.email).await?;

  match user.0 {
    Some(user) => {
      let user: User = serde_json::from_value(user)?;
      let parsed_hash = PasswordHash::new(&user.password)?;
      let argon2 = Argon2::default();
      if argon2
        .verify_password(login_user.password.as_bytes(), &parsed_hash)
        .is_ok()
      {
        Identity::login(&request.extensions(), user.id.to_string())?;
        ApiResponse(Ok(ApiData(Some(json!(user.id)))))
      } else {
        ApiResponse(Err(ApiError {
          error_kind: ApiErrorKind::Unauthorized,
          debug_info: "Invalid password".to_string(),
        }))
      }
    }
    None => ApiResponse(Err(ApiError {
      error_kind: ApiErrorKind::Unauthorized,
      debug_info: "Invalid email".to_string(),
    })),
  }
}

async fn get_all<R: UserRepository>(repo: web::Data<R>) -> ApiResponse {
  repo.get_users().await
}

async fn get<R: UserRepository>(
  user_id: web::Path<i64>,
  repo: web::Data<R>,
) -> ApiResponse {
  repo.get_user_by_id(&user_id).await
}

async fn post<R: UserRepository>(
  create_user: web::Json<CreateUser>,
  repo: web::Data<R>,
) -> ApiResponse {
  let salt = SaltString::generate(&mut OsRng);
  let argon2 = Argon2::default();
  let _password_hash = argon2
    .hash_password(create_user.password.as_bytes(), &salt)?
    .to_string();
  // TODO FIX THIS SHIT IMMEDIETLY
  repo.create_user(&create_user).await
}

async fn put<R: UserRepository>(
  update_user: web::Json<UpdateUser>,
  repo: web::Data<R>,
) -> ApiResponse {
  repo.update_user(&update_user).await
}

async fn delete<R: UserRepository>(
  user_id: web::Path<i64>,
  repo: web::Data<R>,
) -> ApiResponse {
  repo.delete_user(&user_id).await
}
