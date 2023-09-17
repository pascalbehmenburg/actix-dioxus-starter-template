use actix_web::web::ServiceConfig;
use shuttle_actix_web::ShuttleActixWeb;
use api_lib::health::health;

#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_shared_db::Postgres()] pool: sqlx::PgPool,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let pool = actix_web::web::Data::new(pool);

    let config = move |cfg: &mut ServiceConfig| {
        cfg.app_data(pool).service(health);
    };

    Ok(config.into())
}
