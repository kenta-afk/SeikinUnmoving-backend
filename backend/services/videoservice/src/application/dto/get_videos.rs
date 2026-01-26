use crate::domain::models::video::Video;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetVideosDto {
    pub videos: Vec<Video>,
}

impl GetVideosDto {
    pub fn new(videos: Vec<Video>) -> Self {
        Self { videos }
    }
}
