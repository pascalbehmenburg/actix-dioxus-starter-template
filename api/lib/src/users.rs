// this files purpose is to provide a template for new routes
use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse,
};

pub fn service(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/v1/users")
            // get all
            .route("", web::get().to(get_all))
            // get by id
            .route("/{user_id}", web::get().to(get))
            // new
            .route("", web::post().to(post))
            // update
            .route("", web::put().to(put))
            // delete
            .route("/{user_id}", web::delete().to(delete)),
    );
}

async fn get_all() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn get() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn post() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn put() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn delete() -> HttpResponse {
    HttpResponse::Ok().finish()
}
