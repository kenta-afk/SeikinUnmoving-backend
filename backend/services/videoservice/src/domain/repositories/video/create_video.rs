use crate::domain::models::id::VideoId;
use chrono::{DateTime, Utc};

pub struct CreateVideo {
    pub id: VideoId,
    pub youtube_url: String,
    pub title: Option<String>,
    pub duration_seconds: Option<i64>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

impl CreateVideo {
    pub fn new(
        id: VideoId,
        youtube_url: String,
        title: Option<String>,
        duration_seconds: Option<i64>,
    ) -> Self {
        Self {
            id,
            youtube_url,
            title,
            duration_seconds,
            is_active: true,
            created_at: Utc::now(),
        }
    }
}
