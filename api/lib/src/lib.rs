use actix_web::{guard, web, HttpResponse};

pub mod health;
pub mod todos;

pub fn service(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(
        web::resource("/user/{name}")
            .name("user_detail")
            .guard(guard::Header("content-type", "application/json"))
            .route(web::get().to(HttpResponse::Ok)),
    );
}
