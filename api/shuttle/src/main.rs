use actix_identity::IdentityMiddleware;
use actix_session::SessionMiddleware;
use actix_web::{
  cookie::{Key, SameSite},
  middleware,
  web::{self, ServiceConfig},
};
use api_lib::{
  postgres_session_store::PostgresSessionStore,
  todo_repository::{self, PostgresTodoRepository},
  user_repository::{self, PostgresUserRepository},
};
use shuttle_actix_web::ShuttleActixWeb;

#[macro_use]
extern crate dotenv_codegen;

#[shuttle_runtime::main]
async fn actix_web(
  #[shuttle_shared_db::Postgres(
        local_uri = dotenv!("DATABASE_URL")
    )]
  pool: sqlx::PgPool,
  //#[shuttle_static_folder::StaticFolder(folder = "static")] static_folder: PathBuf,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
  let todo_repository =
    todo_repository::PostgresTodoRepository::new(pool.clone());
  let todo_repository = actix_web::web::Data::new(todo_repository);

  let user_repository =
    user_repository::PostgresUserRepository::new(pool.clone());
  let user_repository = actix_web::web::Data::new(user_repository);

  let session_store = PostgresSessionStore::new(pool.clone());
  let signing_key = Key::from(dotenv!("SIGNING_KEY").as_bytes());

  let config = move |cfg: &mut ServiceConfig| {
    cfg.service(
      web::scope("/api")
        .wrap(middleware::Logger::default())
        .wrap(middleware::Compress::default())
        .wrap(IdentityMiddleware::default())
        .wrap(
          SessionMiddleware::builder(session_store, signing_key)
            // allow the cookie to be accessed from javascript
            .cookie_http_only(false)
            // allow the cookie only from the current domain
            .cookie_same_site(SameSite::Strict)
            .build(),
        )
        .app_data(todo_repository)
        .app_data(user_repository)
        .configure(api_lib::health::service)
        .configure(api_lib::todo::service::<PostgresTodoRepository>)
        .configure(api_lib::user::service::<PostgresUserRepository>),
    );
    //.service(
    //    actix_files::Files::new("/", static_folder)
    //        .show_files_listing()
    //        .index_file("index.html"),
    //);
  };

  Ok(config.into())
}
