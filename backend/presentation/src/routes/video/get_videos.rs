use crate::state::VideoServiceState;
use axum::{Json, extract::State, http::StatusCode};
use serde::Serialize;
use utoipa::ToSchema;
use videoservice::{GetVideosCommand, Video, VideoService};

#[derive(Serialize, ToSchema)]
pub struct GetVideosResponse {
    pub videos: Vec<Video>,
}

#[utoipa::path(
    get,
    path = "/api/videos",
    tag = "video",
    responses(
        (status = 200, description = "Videos retrieved successfully", body = GetVideosResponse),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_videos<T>(
    State(VideoServiceState(service)): State<VideoServiceState<T>>,
) -> Result<Json<GetVideosResponse>, StatusCode>
where
    T: VideoService,
{
    let command = GetVideosCommand::new();

    match service.get_videos(command).await {
        Ok(dto) => Ok(Json(GetVideosResponse { videos: dto.videos })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
