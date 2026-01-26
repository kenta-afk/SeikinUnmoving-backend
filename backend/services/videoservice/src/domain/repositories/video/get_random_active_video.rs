use crate::domain::models::video::Video;

pub struct GetRandomActiveVideo {
    pub video: Video,
}

impl GetRandomActiveVideo {
    pub fn new(video: Video) -> Self {
        Self { video }
    }
}
