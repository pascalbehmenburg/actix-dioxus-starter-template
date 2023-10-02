use crate::repository::todo::TodoRepository;
use crate::{routes, util::response::JsonResponse};
use actix_identity::Identity;
use actix_web::web::{self, ServiceConfig};
use shared::models::{CreateTodoForm, Todo, UpdateTodo};

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

async fn get_all<R: TodoRepository>(repo: web::Data<R>) -> JsonResponse {
  repo.get_todos().await
}

async fn get<R: TodoRepository>(
  todo_id: web::Path<i64>,
  repo: web::Data<R>,
) -> JsonResponse {
  repo.get_todo(&todo_id).await
}

async fn post<R: TodoRepository>(
  create_todo_form: web::Json<CreateTodoForm>,
  repo: web::Data<R>,
  user: Identity,
) -> JsonResponse {
  let owner = routes::get_identity_id(user).await?;

  let todo = Todo {
    title: create_todo_form.title.clone(),
    description: create_todo_form.description.clone(),
    owner,
    ..Default::default()
  };

  repo.create_todo(&todo).await
}

async fn put<R: TodoRepository>(
  update_todo: web::Json<UpdateTodo>,
  repo: web::Data<R>,
  user: Identity,
) -> JsonResponse {
  let owner = routes::get_identity_id(user).await?;
  let update_todo = UpdateTodo {
    owner: owner,
    ..update_todo.into_inner()
  };

  repo.update_todo(&update_todo).await
}

async fn delete<R: TodoRepository>(
  todo_id: web::Path<i64>,
  repo: web::Data<R>,
) -> JsonResponse {
  repo.delete_todo(&todo_id).await
}
