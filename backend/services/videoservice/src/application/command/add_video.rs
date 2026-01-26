use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AddVideoCommand {
    pub youtube_url: String,
    pub title: Option<String>,
    pub duration_seconds: Option<i64>,
}

impl AddVideoCommand {
    pub fn new(youtube_url: String, title: Option<String>, duration_seconds: Option<i64>) -> Self {
        Self {
            youtube_url,
            title,
            duration_seconds,
        }
    }
}
