use std::net::SocketAddr;
use axum::Router;
use axum::routing::{get, post};
use sqlx::sqlite::SqlitePoolOptions;

use crate::handlers::{create_url, list_urls, delete_slug, redirect_url};

pub mod models;
pub mod handlers;

#[tokio::main]
async fn main() {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite://db.sqlite".to_string());

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create db pool.");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations.");

    let app = Router::new()
        .route("/urls", post(create_url).get(list_urls))
        .route("/{slug}", get(redirect_url).delete(delete_slug))
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app)
        .await
        .unwrap();
}
