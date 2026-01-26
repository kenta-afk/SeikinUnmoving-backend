use crate::domain::models::{error::DbError, id::VideoId, video::Video};
use crate::domain::repositories::video::{
    create_video::CreateVideo, get_random_active_video::GetRandomActiveVideo,
    get_videos::GetVideos, video_repository::VideoRepository,
};
use async_trait::async_trait;
use chrono::{DateTime, Utc};

pub struct VideoRepositoryImpl {
    #[cfg(not(target_arch = "wasm32"))]
    pool: sqlx::SqlitePool,
}

impl VideoRepositoryImpl {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self { pool }
    }
}

impl Clone for VideoRepositoryImpl {
    fn clone(&self) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        {
            Self {
                pool: self.pool.clone(),
            }
        }
        #[cfg(target_arch = "wasm32")]
        {
            Self {}
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[async_trait]
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
