use crate::domain::{
    models::error::DbError,
    repositories::video::{
        create_video::CreateVideo, get_random_active_video::GetRandomActiveVideo,
        get_videos::GetVideos, video_repository::VideoRepository,
    },
};

// WASM環境用の実装
#[cfg(target_arch = "wasm32")]
#[derive(Clone, Default)]
pub struct VideoRepositoryImpl {}

#[cfg(target_arch = "wasm32")]
impl VideoRepositoryImpl {
    pub fn new() -> Self {
        Self {}
    }
}

#[cfg(target_arch = "wasm32")]
#[async_trait::async_trait(?Send)]
impl VideoRepository for VideoRepositoryImpl {
    async fn create(&self, _video: CreateVideo) -> Result<(), DbError> {
        Err(DbError::Generic("Not implemented for WASM".to_string()))
    }

    async fn get_all(&self) -> Result<GetVideos, DbError> {
        Err(DbError::Generic("Not implemented for WASM".to_string()))
    }

    async fn get_random_active(&self) -> Result<GetRandomActiveVideo, DbError> {
        Err(DbError::Generic("Not implemented for WASM".to_string()))
    }
}

// ローカル開発環境用の実装（SQLXを使用）
#[cfg(not(target_arch = "wasm32"))]
use crate::domain::models::{id::VideoId, video::Video};
#[cfg(not(target_arch = "wasm32"))]
use chrono::{DateTime, Utc};

#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone)]
pub struct VideoRepositoryImpl {
    pool: sqlx::SqlitePool,
}

#[cfg(not(target_arch = "wasm32"))]
impl VideoRepositoryImpl {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self { pool }
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[async_trait::async_trait]
impl VideoRepository for VideoRepositoryImpl {
    async fn create(&self, video: CreateVideo) -> Result<(), DbError> {
        let id = video.id.as_str();
        let created_at = video.created_at.to_rfc3339();
        let is_active = if video.is_active { 1 } else { 0 };

        sqlx::query!(
            r#"
            INSERT INTO videos (id, youtube_url, title, duration_seconds, is_active, created_at)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
            id,
            video.youtube_url,
            video.title,
            video.duration_seconds,
            is_active,
            created_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| DbError::Generic(e.to_string()))?;

        Ok(())
    }

    async fn get_all(&self) -> Result<GetVideos, DbError> {
        let records = sqlx::query!(
            r#"
            SELECT id, youtube_url, title, duration_seconds, is_active, created_at
            FROM videos
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DbError::Generic(e.to_string()))?;

        let videos: Vec<Video> = records
            .into_iter()
            .map(|r| {
                Video::new(
                    VideoId::new(r.id),
                    r.youtube_url,
                    r.title,
                    r.duration_seconds,
                    r.is_active != 0,
                    r.created_at
                        .parse::<DateTime<Utc>>()
                        .unwrap_or_else(|_| Utc::now()),
                )
            })
            .collect();

        Ok(GetVideos::new(videos))
    }

    async fn get_random_active(&self) -> Result<GetRandomActiveVideo, DbError> {
        let record = sqlx::query!(
            r#"
            SELECT id, youtube_url, title, duration_seconds, is_active, created_at
            FROM videos
            WHERE is_active = 1
            ORDER BY RANDOM()
            LIMIT 1
            "#
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DbError::Generic(e.to_string()))?;

        match record {
            Some(r) => {
                let video = Video::new(
                    VideoId::new(r.id),
                    r.youtube_url,
                    r.title,
                    r.duration_seconds,
                    r.is_active != 0,
                    r.created_at
                        .parse::<DateTime<Utc>>()
                        .unwrap_or_else(|_| Utc::now()),
                );
                Ok(GetRandomActiveVideo::new(video))
            }
            None => Err(DbError::NotFound),
        }
    }
}
