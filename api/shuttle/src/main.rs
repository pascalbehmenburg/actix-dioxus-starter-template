use actix_web::web::ServiceConfig;
use api_lib::todo_repository;
use shuttle_actix_web::ShuttleActixWeb;

#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_shared_db::Postgres()] pool: sqlx::PgPool,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let todo_repository = todo_repository::PostgresTodoRepository::new(pool);
    let todo_repository = actix_web::web::Data::new(todo_repository);

    let config = move |cfg: &mut ServiceConfig| {
        cfg.app_data(todo_repository)
            .configure(api_lib::health::service)
            .configure(api_lib::todos::service);
    };

    Ok(config.into())
}
