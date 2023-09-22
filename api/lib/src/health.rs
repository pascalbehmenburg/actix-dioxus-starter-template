use actix_web::{web, HttpResponse};

pub const API_VERSION: &str = "v0.0.1";

async fn health() -> HttpResponse {
  tracing::info!("Returning health status");
  HttpResponse::Ok()
    .append_header(("version", API_VERSION))
    .finish()
}

pub fn service(cfg: &mut actix_web::web::ServiceConfig) {
  cfg.route("/health", web::get().to(health));
}
