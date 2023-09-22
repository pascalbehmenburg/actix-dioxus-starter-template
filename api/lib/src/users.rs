use actix_web::web::{self, ServiceConfig};
use shared::models::{CreateUser, UpdateUser};

use argon2::{
  password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
  Argon2,
};

use crate::{response::ApiResponse, user_repository::UserRepository};

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
      .route("/{user_id}", web::delete().to(delete::<R>)),
  );
}

async fn get_all<R: UserRepository>(repo: web::Data<R>) -> ApiResponse {
  repo.get_users().await
}

async fn get<R: UserRepository>(
  user_id: web::Path<i64>,
  repo: web::Data<R>,
) -> ApiResponse {
  repo.get_user(&user_id).await
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
