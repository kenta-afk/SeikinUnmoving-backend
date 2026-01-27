use super::{
    create_video::CreateVideo, get_random_active_video::GetRandomActiveVideo, get_videos::GetVideos,
};
use crate::domain::models::error::DbError;
use async_trait::async_trait;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
#[cfg(not(target_arch = "wasm32"))]
pub trait VideoRepository: Send + Sync + 'static {
    async fn create(&self, video: CreateVideo) -> Result<(), DbError>;
    async fn get_all(&self) -> Result<GetVideos, DbError>;
    async fn get_random_active(&self) -> Result<GetRandomActiveVideo, DbError>;
}

#[cfg_attr(test, mockall::automock)]
#[async_trait(?Send)]
#[cfg(target_arch = "wasm32")]
pub trait VideoRepository: Sync {
    async fn create(&self, video: CreateVideo) -> Result<(), DbError>;
    async fn get_all(&self) -> Result<GetVideos, DbError>;
    async fn get_random_active(&self) -> Result<GetRandomActiveVideo, DbError>;
}
