use actix_web::web::{self, ServiceConfig};
use shared::models::{CreateTodo, UpdateTodo};

use crate::{repository::TodoRepository, util::Response};

pub fn service<R: TodoRepository>(cfg: &mut ServiceConfig) {
  cfg.service(
    web::scope("/v1/todos")
      // get all
      .route("", web::get().to(get_all::<R>))
      // get by id
      .route("/{todo_id}", web::get().to(get::<R>))
      // post (create)
      .route("", web::post().to(post::<R>))
      // update
      .route("", web::put().to(put::<R>))
      // delete
      .route("/{todo_id}", web::delete().to(delete::<R>)),
  );
}

async fn get_all<R: TodoRepository>(repo: web::Data<R>) -> Response {
  repo.get_todos().await
}

async fn get<R: TodoRepository>(
  todo_id: web::Path<i64>,
  repo: web::Data<R>,
) -> Response {
  repo.get_todo(&todo_id).await
}

async fn post<R: TodoRepository>(
  create_todo: web::Json<CreateTodo>,
  repo: web::Data<R>,
) -> Response {
  repo.create_todo(&create_todo).await
}

async fn put<R: TodoRepository>(
  update_todo: web::Json<UpdateTodo>,
  repo: web::Data<R>,
) -> Response {
  repo.update_todo(&update_todo).await
}

async fn delete<R: TodoRepository>(
  todo_id: web::Path<i64>,
  repo: web::Data<R>,
) -> Response {
  repo.delete_todo(&todo_id).await
}
