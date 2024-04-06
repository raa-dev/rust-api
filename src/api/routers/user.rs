use actix_web::web;
use crate::api::controllers;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("", web::get().to(controllers::user::find_all))
            .route("/{id}", web::get().to(controllers::user::find_by_id))
            .route("", web::post().to(controllers::user::create))
            .route("/{id}", web::put().to(controllers::user::update))
            // .route("/{id}", web::delete().to(controllers::user::delete))
    );
}
