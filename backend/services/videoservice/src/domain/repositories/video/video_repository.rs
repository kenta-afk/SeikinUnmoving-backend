use super::{
    create_video::CreateVideo, get_random_active_video::GetRandomActiveVideo, get_videos::GetVideos,
};
use crate::domain::models::error::DbError;
use async_trait::async_trait;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait VideoRepository: Send + Sync {
    async fn create(&self, video: CreateVideo) -> Result<(), DbError>;
    async fn get_all(&self) -> Result<GetVideos, DbError>;
    async fn get_random_active(&self) -> Result<GetRandomActiveVideo, DbError>;
}
