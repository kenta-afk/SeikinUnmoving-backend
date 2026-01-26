use crate::state::VideoServiceState;
use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use videoservice::{AddVideoCommand, VideoService};

#[derive(Deserialize, ToSchema)]
pub struct AddVideoRequest {
    pub youtube_url: String,
    pub title: Option<String>,
    pub duration_seconds: Option<i64>,
}

#[derive(Serialize, ToSchema)]
pub struct AddVideoResponse {
    pub youtube_url: String,
    pub title: Option<String>,
    pub duration_seconds: Option<i64>,
}

#[utoipa::path(
    post,
    path = "/api/videos",
    tag = "video",
    request_body = AddVideoRequest,
    responses(
        (status = 200, description = "Video successfully added", body = AddVideoResponse),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn add_video<T>(
    State(VideoServiceState(service)): State<VideoServiceState<T>>,
    Json(payload): Json<AddVideoRequest>,
) -> Result<Json<AddVideoResponse>, StatusCode>
where
    T: VideoService,
{
    let command =
        AddVideoCommand::new(payload.youtube_url, payload.title, payload.duration_seconds);

    match service.add_video(command).await {
        Ok(dto) => Ok(Json(AddVideoResponse {
            youtube_url: dto.youtube_url,
            title: dto.title,
            duration_seconds: dto.duration_seconds,
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
