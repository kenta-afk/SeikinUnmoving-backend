pub mod add_video;
pub mod get_random_video;
pub mod get_videos;

pub use add_video::{add_video, AddVideoRequest, AddVideoResponse, __path_add_video};
pub use get_random_video::{
    get_random_video, GetRandomVideoResponse, __path_get_random_video,
};
pub use get_videos::{get_videos, GetVideosResponse, __path_get_videos};
