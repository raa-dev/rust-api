use uuid::Uuid;
use sqlx::PgPool;
use crate::api::models::user::User;

pub async fn find_all(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    let users = sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(pool)
        .await?;
    Ok(users)
}

pub async fn find_by_id(pool: &PgPool, id: String) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(&id)
        .fetch_one(pool)
        .await?;
    Ok(user)
}

pub async fn create(pool: &PgPool, name: &str, email: &str) -> Result<User, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    let user = sqlx::query_as::<_, User>("INSERT INTO users (id, name, email) VALUES ($1, $2, $3) RETURNING *")
        .bind(id)
        .bind(name)
        .bind(email)
        .fetch_one(pool)
        .await?;
    Ok(user)
}

pub async fn update(pool: &PgPool, id: String, name: &str, email: &str) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as::<_, User>("UPDATE users SET name = $2, email = $3 WHERE id = $1 RETURNING *")
        .bind(name)
        .bind(email)
        .bind(&id)
        .fetch_one(pool)
        .await?;
    Ok(user)
}

pub async fn delete(pool: &PgPool, id: String) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(&id)
        .execute(pool)
        .await?;
    Ok(())
}