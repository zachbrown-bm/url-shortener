use crate::types::UrlStore;
use crate::models::{CreateUrl, UrlResponse};

use axum::{extract::State, extract::Path, http::StatusCode, Json};
use axum::response::Redirect;

pub async fn create_url(
    State(store): State<UrlStore>,
    Json(payload): Json<CreateUrl>,
) -> (StatusCode, Json<UrlResponse>) {
    let slug = nanoid::nanoid!(6);
    store.write().unwrap().insert(slug.clone(), payload.url.clone());

    (
        StatusCode::CREATED,
        Json(UrlResponse {
            slug,
            url: payload.url,
        }),
    )
}

pub async fn redirect_url(
    State(store): State<UrlStore>,
    Path(slug): Path<String>,
) -> Result<Redirect, StatusCode> {
    store
        .read()
        .unwrap()
        .get(&slug)
        .map(|url| Redirect::permanent(url))
        .ok_or(StatusCode::NOT_FOUND)
}

pub async fn list_urls(
    State(store): State<UrlStore>,
) -> Json<Vec<UrlResponse>> {
    let store = store.read().unwrap();
    let urls: Vec<UrlResponse> = store
        .iter()
        .map(|(slug, url)| UrlResponse {
            slug: slug.clone(),
            url: url.clone(),
        })
        .collect();
    Json(urls)
}