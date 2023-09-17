use actix_web::{HttpResponse, web};

async fn health() -> HttpResponse {
    tracing::info!("Returning health status");
    HttpResponse::Ok()
        .append_header(("version", "0.0.1"))
        .finish()
}

pub fn service(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.route("/health", web::get().to(health));
}