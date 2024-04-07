mod api;
use sqlx;
use std::env;
use dotenv::dotenv;
use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let base_url = env::var("BASE_URL").expect("BASE_URL must be set");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = sqlx::PgPool::connect(&database_url).await.unwrap();
    let pool = web::Data::new(pool);
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();
    tracing::info!("Starting server at {}", &base_url);
    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .service(
                web::scope("/api")
                    .configure(api::routers::user::init)
            )
    })
    .bind(&base_url)
    .unwrap()
    .run()
    .await
}



