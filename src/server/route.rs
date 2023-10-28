use actix_web::{web, HttpResponse};

pub fn chris_cfg_fn(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/test")
            .route(web::get().to(|| async { HttpResponse::Ok().body("chris test") }))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}

pub fn mongo_cfg_fn(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/create")
            .route(web::post().to(|| async { HttpResponse::Ok().body("mongo create") }))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    ).service(
        web::resource("/read")
            .route(web::get().to(|| async { HttpResponse::Ok().body("mongo read") }))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    ).service(
        web::resource("/update")
            .route(web::put().to(|| async { HttpResponse::Ok().body("mongo update") }))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    ).service(
        web::resource("/delete")
            .route(web::delete().to(|| async { HttpResponse::Ok().body("mongo delete") }))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}
