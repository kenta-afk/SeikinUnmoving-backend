use crate::domain::models::video::Video;

pub struct GetVideos {
    pub videos: Vec<Video>,
}

impl GetVideos {
    pub fn new(videos: Vec<Video>) -> Self {
        Self { videos }
    }
}
