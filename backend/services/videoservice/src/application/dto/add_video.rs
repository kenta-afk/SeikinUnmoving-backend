use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AddVideoDto {
    pub youtube_url: String,
    pub title: Option<String>,
    pub duration_seconds: Option<i64>,
}
