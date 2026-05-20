use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize };

#[derive(Deserialize)]
pub struct CreateUrl {
    pub url: String,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct UrlResponse {
    pub slug: String,
    pub url: String,
    pub created_at: NaiveDateTime,
}
