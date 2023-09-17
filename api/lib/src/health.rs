use actix_web::{{get, HttpResponse}};

#[get("/health")]
async fn health() -> HttpResponse {
    tracing::info!("Returning health status");
    HttpResponse::Ok()
        .append_header(("version", "0.0.1"))
        .finish()
}