
use sqlx::PgPool;
use crate::api::models::user::User;
use crate::api::repositories::user as user_repository;

pub async fn find_all(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    user_repository::find_all(pool).await
}

pub async fn find_by_id(pool: &PgPool, id: String) -> Result<User, sqlx::Error> {
    user_repository::find_by_id(pool, id).await
}

pub async fn create(pool: &PgPool, name: &str, email: &str) -> Result<User, sqlx::Error> {
    user_repository::create(pool, name, email).await
}

pub async fn update(pool: &PgPool, id: String, name: &str, email: &str) -> Result<User, sqlx::Error> {
    user_repository::update(pool, id, name, email).await
}

// pub async fn delete(pool: &PgPool, id: String) -> Result<(), sqlx::Error> {
//     user_repository::delete(pool, id).await
// }
