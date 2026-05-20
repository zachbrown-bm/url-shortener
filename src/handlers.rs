use crate::models::{CreateUrl, UrlResponse};

use axum::{extract::State, extract::Path, http::StatusCode, Json};
use axum::response::Redirect;
use sqlx::SqlitePool;

pub async fn create_url(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateUrl>,
) -> Result<(StatusCode, Json<UrlResponse>), StatusCode> {
    let slug = nanoid::nanoid!(6);

    let url_row = sqlx::query_as::<_, UrlResponse>(
        "INSERT INTO urls (slug, url) VALUES (?, ?) RETURNING slug, url, created_at"
        )
        .bind(&slug)
        .bind(&payload.url)
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(url_row)))
}

pub async fn redirect_url(
    State(pool): State<SqlitePool>,
    Path(slug): Path<String>,
) -> Result<Redirect, StatusCode> {
    let row = sqlx::query_as::<_, UrlResponse>(
        "SELECT slug, url, created_at FROM urls WHERE slug = ?"
        )
        .bind(&slug)
        .fetch_optional(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    row.map(|r| Redirect::temporary(&r.url))
        .ok_or(StatusCode::NOT_FOUND)
}

pub async fn delete_slug(
    State(pool): State<SqlitePool>,
    Path(slug): Path<String>,
) -> Result<StatusCode, StatusCode> {
    sqlx::query("DELETE FROM urls WHERE slug = ?")
        .bind(&slug)
        .execute(&pool)
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn list_urls(
    State(pool): State<SqlitePool>,
) -> Result<Json<Vec<UrlResponse>>, StatusCode> {
    let urls = sqlx::query_as::<_, UrlResponse>(
        "SELECT slug, url, created_at FROM urls ORDER BY created_at DESC"
    )
        .fetch_all(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(urls))
}