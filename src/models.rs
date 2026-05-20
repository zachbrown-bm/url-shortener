use serde::{ Deserialize, Serialize };

#[derive(Deserialize)]
pub struct CreateUrl {
    pub url: String,
}

#[derive(Serialize)]
pub struct UrlResponse {
    pub slug: String,
    pub url: String,
}
