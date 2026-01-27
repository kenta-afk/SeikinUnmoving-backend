use videoservice::domain::{
    models::{error::DbError, id::VideoId, video::Video},
    repositories::video::{
        create_video::CreateVideo, get_random_active_video::GetRandomActiveVideo,
        get_videos::GetVideos, video_repository::VideoRepository,
    },
};
use worker::d1::D1Database;
use std::sync::Arc;
use serde::Deserialize;
use chrono::{DateTime, Utc};

// D1から取得したビデオデータの中間構造体
#[derive(Debug, Deserialize)]
struct VideoRow {
    id: String,
    youtube_url: String,
    title: Option<String>,
    duration_seconds: Option<i64>,
    is_active: i32, // SQLiteは整数で返す
    created_at: String,
}

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

        let rows: Vec<VideoRow> = result
            .results::<VideoRow>()
            .map_err(|e| DbError::Generic(format!("Failed to deserialize videos: {}", e)))?;

        let videos: Vec<Video> = rows
            .into_iter()
            .map(|row| {
                Video::new(
                    VideoId::new(row.id),
                    row.youtube_url,
                    row.title,
                    row.duration_seconds,
                    row.is_active != 0,
                    row.created_at.parse::<DateTime<Utc>>().unwrap_or_else(|_| Utc::now()),
                )
            })
            .collect();

        Ok(GetVideos { videos })
    }

    async fn get_random_active(&self) -> Result<GetRandomActiveVideo, DbError> {
        let result = self
            .db
            .prepare(
                "SELECT id, youtube_url, title, duration_seconds, is_active, created_at 
                 FROM videos WHERE is_active = 1 ORDER BY RANDOM() LIMIT 1"
            )
            .first::<VideoRow>(None)
            .await
            .map_err(|e| DbError::Generic(e.to_string()))?;

        match result {
            Some(row) => {
                let video = Video::new(
                    VideoId::new(row.id),
                    row.youtube_url,
                    row.title,
                    row.duration_seconds,
                    row.is_active != 0,
                    row.created_at.parse::<DateTime<Utc>>().unwrap_or_else(|_| Utc::now()),
                );
                Ok(GetRandomActiveVideo::new(video))
            }
            None => Err(DbError::Generic("No active videos available".to_string())),
        }
    }
}
