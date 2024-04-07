
use sqlx::PgPool;
use actix_web::{web, HttpResponse};
use crate::api::models::user::User;
use crate::api::services::user as user_service;

pub async fn find_all(pool: web::Data<PgPool>) -> HttpResponse {
    let users = user_service::find_all(&pool).await.unwrap();
    HttpResponse::Ok().json(users)
}

pub async fn find_by_id(pool: web::Data<PgPool>, id: web::Path<String>) -> HttpResponse {
    let user = user_service::find_by_id(&pool, id.into_inner()).await.unwrap();
    HttpResponse::Ok().json(user)
}

pub async fn create(pool: web::Data<PgPool>, user: web::Json<User>) -> HttpResponse {
    let user = user_service::create(&pool, &user.name, &user.email).await.unwrap();
    HttpResponse::Ok().json(user)
}

pub async fn update(pool: web::Data<PgPool>, id: web::Path<String>, user: web::Json<User>) -> HttpResponse {
    let user = user_service::update(&pool, id.into_inner(), &user.name, &user.email).await.unwrap();
    HttpResponse::Ok().json(user)
}

pub async fn delete(pool: web::Data<PgPool>, id: web::Path<String>) -> HttpResponse {
    user_service::delete(&pool, id.into_inner()).await.unwrap();
    HttpResponse::Ok().finish()
}
