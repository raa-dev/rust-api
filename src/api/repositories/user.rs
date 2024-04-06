use crate::api::models::user::User;
use sqlx::PgPool;

pub async fn find_all(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    let users = sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(pool)
        .await?;
    Ok(users)
}

pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as::<_, User>(format!("SELECT * FROM users WHERE id = {}", id).as_str())
        .bind(&id)
        .fetch_one(pool)
        .await?;
    Ok(user)
}

pub async fn create(pool: &PgPool, name: &str, email: &str) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as::<_, User>(format!("INSERT INTO users (name, email) VALUES ({}, {}) RETURNING *", name, email).as_str())
        .bind(name)
        .bind(email)
        .fetch_one(pool)
        .await?;
    Ok(user)
}

pub async fn update(pool: &PgPool, id: i32, name: &str, email: &str) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as::<_, User>(format!("UPDATE users SET name = {}, email = {} WHERE id = {} RETURNING *", name, email, id).as_str())
        .bind(name)
        .bind(email)
        .bind(&id)
        .fetch_one(pool)
        .await?;
    Ok(user)
}

// pub async fn delete(pool: &PgPool, id: i32) -> Result<(), sqlx::Error> {
//     // sqlx::query("DELETE FROM users WHERE id = $1")
//     //     .bind(&id)
//     //     .execute(())
//     //     .await?;
//     Ok(())
// }