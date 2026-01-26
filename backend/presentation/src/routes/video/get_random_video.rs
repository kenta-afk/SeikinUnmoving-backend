use crate::state::VideoServiceState;
use axum::{extract::State, http::StatusCode, Json};
use serde::Serialize;
use utoipa::ToSchema;
use videoservice::{GetRandomVideoCommand, Video, VideoService};

#[derive(Serialize, ToSchema)]
pub struct GetRandomVideoResponse {
    pub video: Video,
}

#[utoipa::path(
    get,
    path = "/api/videos/random",
    tag = "video",
    responses(
        (status = 200, description = "Random video retrieved successfully", body = GetRandomVideoResponse),
        (status = 404, description = "No active videos available"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_random_video<T>(
    State(VideoServiceState(service)): State<VideoServiceState<T>>,
) -> Result<Json<GetRandomVideoResponse>, StatusCode>
where
    T: VideoService,
{
    let command = GetRandomVideoCommand::new();

    match service.get_random_video(command).await {
        Ok(dto) => Ok(Json(GetRandomVideoResponse { video: dto.video })),
        Err(e) => match e {
            videoservice::ServiceError::NoActiveVideos
            | videoservice::ServiceError::VideoNotFound => Err(StatusCode::NOT_FOUND),
            _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
    }
}
