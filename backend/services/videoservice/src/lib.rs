use crate::infrastructure::{
    adapters::uuid_service::UuidServiceImpl, repositories::video::repository::VideoRepositoryImpl,
};
use thiserror::Error;

mod application;
mod domain;
pub mod infrastructure;

pub use application::{
    command::{
        add_video::AddVideoCommand, get_random_video::GetRandomVideoCommand,
        get_videos::GetVideosCommand,
    },
    dto::{
        add_video::AddVideoDto, get_random_video::GetRandomVideoDto, get_videos::GetVideosDto,
    },
    ports::uuid_service::UuidService,
    videoservice::{VideoService, VideoServiceImpl},
};
pub use domain::{
    models::{error::DbError, id::VideoId, video::Video},
    repositories::video::{
        create_video::CreateVideo, get_random_active_video::GetRandomActiveVideo,
        get_videos::GetVideos, video_repository::VideoRepository,
    },
};

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("Database error: {0}")]
    Database(#[from] DbError),
    #[error("video not found")]
    VideoNotFound,
    #[error("no active videos available")]
    NoActiveVideos,
}

pub type ConcreteVideoService = VideoServiceImpl<VideoRepositoryImpl, UuidServiceImpl>;

// 開発環境用のビルド関数
#[cfg(not(target_arch = "wasm32"))]
pub async fn build_service(database_url: &str) -> Result<ConcreteVideoService, ServiceError> {
    use sqlx::sqlite::SqlitePoolOptions;

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .map_err(|e| ServiceError::Database(DbError::Generic(e.to_string())))?;

    let video_repo = VideoRepositoryImpl::new(pool);
    let uuid_service = UuidServiceImpl;

    Ok(VideoServiceImpl::new(video_repo, uuid_service))
}
