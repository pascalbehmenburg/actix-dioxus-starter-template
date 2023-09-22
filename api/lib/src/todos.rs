use actix_web::{
  web::{self, ServiceConfig},
  HttpResponse,
};
use shared::models::{CreateTodo, UpdateTodo};

use crate::todo_repository::TodoRepository;

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

async fn get_all<R: TodoRepository>(repo: web::Data<R>) -> HttpResponse {
  match repo.get_todos().await {
    Ok(todos) => HttpResponse::Ok().json(todos),
    Err(e) => {
      HttpResponse::NotFound().body(format!("Internal server error: {:?}", e))
    }
  }
}

async fn get<R: TodoRepository>(
  todo_id: web::Path<i64>,
  repo: web::Data<R>,
) -> HttpResponse {
  match repo.get_todo(&todo_id).await {
    Ok(todo) => HttpResponse::Ok().json(todo),
    Err(e) => {
      HttpResponse::NotFound().body(format!("Internal server error: {:?}", e))
    }
  }
}

async fn post<R: TodoRepository>(
  create_todo: web::Json<CreateTodo>,
  repo: web::Data<R>,
) -> HttpResponse {
  match repo.create_todo(&create_todo).await {
    Ok(todo) => {
      tracing::info!("{:?}", todo);
      HttpResponse::Ok().json(todo)
    }
    Err(e) => HttpResponse::InternalServerError()
      .body(format!("Internal server error: {:?}", e)),
  }
}

async fn put<R: TodoRepository>(
  update_todo: web::Json<UpdateTodo>,
  repo: web::Data<R>,
) -> HttpResponse {
  match repo.update_todo(&update_todo).await {
    Ok(todo) => HttpResponse::Ok().json(todo),
    Err(e) => HttpResponse::InternalServerError()
      .body(format!("Internal server error: {:?}", e)),
  }
}

async fn delete<R: TodoRepository>(
  todo_id: web::Path<i64>,
  repo: web::Data<R>,
) -> HttpResponse {
  match repo.delete_todo(&todo_id).await {
    Ok(todo_id) => HttpResponse::Ok().json(todo_id),
    Err(e) => HttpResponse::InternalServerError()
      .body(format!("Internal server error: {:?}", e)),
  }
}
