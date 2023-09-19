use std::path::PathBuf;

use actix_web::web::{self, ServiceConfig};
use api_lib::todo_repository::{self, PostgresTodoRepository};
use shuttle_actix_web::ShuttleActixWeb;

#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_shared_db::Postgres(
        local_uri = "postgres://postgres:postgres@localhost:19723/lentserver"
    )]
    pool: sqlx::PgPool,
    #[shuttle_static_folder::StaticFolder(folder = "static")] static_folder: PathBuf,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let todo_repository = todo_repository::PostgresTodoRepository::new(pool);
    let todo_repository = actix_web::web::Data::new(todo_repository);

    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(
            web::scope("/api")
                .app_data(todo_repository)
                .configure(api_lib::health::service)
                .configure(api_lib::todos::service::<PostgresTodoRepository>),
        )
        .service(
            actix_files::Files::new("/", static_folder)
                .show_files_listing()
                .index_file("index.html"),
        );
    };

    Ok(config.into())
}
