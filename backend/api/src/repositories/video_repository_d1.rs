use videoservice::domain::{
    models::{error::DbError, video::Video},
    repositories::video::{
        create_video::CreateVideo, get_random_active_video::GetRandomActiveVideo,
        get_videos::GetVideos, video_repository::VideoRepository,
    },
};
use worker::d1::D1Database;
use std::sync::Arc;

#[derive(Clone)]
pub struct VideoRepositoryD1 {
    db: Arc<D1Database>,
}

impl VideoRepositoryD1 {
    pub fn new(db: D1Database) -> Self {
        Self { db: Arc::new(db) }
    }
}

#[async_trait::async_trait(?Send)]
impl VideoRepository for VideoRepositoryD1 {
    async fn create(&self, video: CreateVideo) -> Result<(), DbError> {
        let id = video.id.as_str();
        let created_at = video.created_at.to_rfc3339();
        let is_active = if video.is_active { 1 } else { 0 };
        
        let query = self
            .db
            .prepare(
                "INSERT INTO videos (id, youtube_url, title, duration_seconds, is_active, created_at) 
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)"
            )
            .bind(&[
                id.into(),
                video.youtube_url.as_str().into(),
                video.title.as_deref().map(|t| t.into()).unwrap_or(worker::wasm_bindgen::JsValue::NULL),
                video.duration_seconds.map(|d| d.into()).unwrap_or(worker::wasm_bindgen::JsValue::NULL),
                is_active.into(),
                created_at.into(),
            ])
            .map_err(|e| DbError::Generic(e.to_string()))?;

        query
            .run()
            .await
            .map_err(|e| DbError::Generic(e.to_string()))?;

        Ok(())
    }

    async fn get_all(&self) -> Result<GetVideos, DbError> {
        let result = self
            .db
            .prepare(
                "SELECT id, youtube_url, title, duration_seconds, is_active, created_at 
                 FROM videos WHERE is_active = 1 ORDER BY created_at DESC"
            )
            .all()
            .await
            .map_err(|e| DbError::Generic(e.to_string()))?;

        let videos: Vec<Video> = result
            .results::<Video>()
            .map_err(|e| DbError::Generic(format!("Failed to deserialize videos: {}", e)))?;

        Ok(GetVideos { videos })
    }

    async fn get_random_active(&self) -> Result<GetRandomActiveVideo, DbError> {
        let result = self
            .db
            .prepare(
                "SELECT id, youtube_url, title, duration_seconds, is_active, created_at 
                 FROM videos WHERE is_active = 1 ORDER BY RANDOM() LIMIT 1"
            )
            .first::<serde_json::Value>(None)
            .await
            .map_err(|e| DbError::Generic(e.to_string()))?;

        match result {
            Some(row) => {
                let video: Video = serde_json::from_value(row)
                    .map_err(|e| DbError::Generic(format!("Failed to deserialize video: {}", e)))?;
                Ok(GetRandomActiveVideo::new(video))
            }
            None => Err(DbError::Generic("No active videos available".to_string())),
        }
    }
}
