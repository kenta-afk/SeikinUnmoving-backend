use super::id::VideoId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[cfg(feature = "utoipa")]
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
pub struct Video {
    pub id: VideoId,
    pub youtube_url: String,
    pub title: Option<String>,
    pub duration_seconds: Option<i64>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

impl Video {
    pub fn new(
        id: VideoId,
        youtube_url: String,
        title: Option<String>,
        duration_seconds: Option<i64>,
        is_active: bool,
        created_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            youtube_url,
            title,
            duration_seconds,
            is_active,
            created_at,
        }
    }
}
