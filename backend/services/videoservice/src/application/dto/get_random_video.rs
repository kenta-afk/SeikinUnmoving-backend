use crate::domain::models::video::Video;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetRandomVideoDto {
    pub video: Video,
}

impl GetRandomVideoDto {
    pub fn new(video: Video) -> Self {
        Self { video }
    }
}
