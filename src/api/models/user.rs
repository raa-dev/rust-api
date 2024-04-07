use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[derive(sqlx::FromRow)]
pub struct User {
    pub id: Option<String>,
    pub name: String,
    pub email: String,
}