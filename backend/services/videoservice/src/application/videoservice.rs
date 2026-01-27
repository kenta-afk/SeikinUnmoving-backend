use super::{
    command::{
        add_video::AddVideoCommand, get_random_video::GetRandomVideoCommand,
        get_videos::GetVideosCommand,
    },
    dto::{add_video::AddVideoDto, get_random_video::GetRandomVideoDto, get_videos::GetVideosDto},
    ports::uuid_service::UuidService,
};
use crate::{
    ServiceError,
    domain::{
        models::id::VideoId,
        repositories::video::{create_video::CreateVideo, video_repository::VideoRepository},
    },
};
use async_trait::async_trait;

#[async_trait]
pub trait VideoService: Send + Sync + 'static {
    async fn add_video(&self, command: AddVideoCommand) -> Result<AddVideoDto, ServiceError>;
    async fn get_videos(&self, command: GetVideosCommand) -> Result<GetVideosDto, ServiceError>;
    async fn get_random_video(
        &self,
        command: GetRandomVideoCommand,
    ) -> Result<GetRandomVideoDto, ServiceError>;
}

pub struct VideoServiceImpl<R, U>
where
    R: VideoRepository,
    U: UuidService,
{
    video_repository: R,
    uuid_service: U,
}

impl<R, U> Clone for VideoServiceImpl<R, U>
where
    R: VideoRepository + Clone,
    U: UuidService + Clone,
{
    fn clone(&self) -> Self {
        Self {
            video_repository: self.video_repository.clone(),
            uuid_service: self.uuid_service.clone(),
        }
    }
}

impl<R, U> VideoServiceImpl<R, U>
where
    R: VideoRepository,
    U: UuidService,
{
    pub fn new(video_repository: R, uuid_service: U) -> Self {
        Self {
            video_repository,
            uuid_service,
        }
    }
}

#[async_trait]
impl<R, U> VideoService for VideoServiceImpl<R, U>
where
    R: VideoRepository + 'static,
    U: UuidService + 'static,
{
    async fn add_video(&self, command: AddVideoCommand) -> Result<AddVideoDto, ServiceError> {
        let video_id = VideoId::new(self.uuid_service.generate());

        let create_video = CreateVideo::new(
            video_id,
            command.youtube_url.clone(),
            command.title.clone(),
            command.duration_seconds,
        );

        self.video_repository.create(create_video).await?;

        Ok(AddVideoDto {
            youtube_url: command.youtube_url,
            title: command.title,
            duration_seconds: command.duration_seconds,
        })
    }

    async fn get_videos(&self, _command: GetVideosCommand) -> Result<GetVideosDto, ServiceError> {
        let result = self.video_repository.get_all().await?;
        Ok(GetVideosDto::new(result.videos))
    }

    async fn get_random_video(
        &self,
        _command: GetRandomVideoCommand,
    ) -> Result<GetRandomVideoDto, ServiceError> {
        let result = self.video_repository.get_random_active().await?;
        Ok(GetRandomVideoDto::new(result.video))
    }
}
