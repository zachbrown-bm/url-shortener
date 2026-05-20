use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};
use axum::Router;
use axum::routing::{get, post};
use crate::handlers::{create_url, list_urls, redirect_url};
use crate::types::UrlStore;

pub mod models;
pub mod handlers;
pub mod types;

#[tokio::main]
async fn main() {
    let store: UrlStore = Arc::new(RwLock::new(HashMap::new()));
    
    let app = Router::new()
        .route("/urls", post(create_url).get(list_urls))
        .route("/{slug}", get(redirect_url))
        .with_state(store);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app)
        .await
        .unwrap();
}
