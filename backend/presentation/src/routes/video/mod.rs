pub mod add_video;
pub mod get_random_video;
pub mod get_videos;

pub use add_video::{__path_add_video, AddVideoRequest, AddVideoResponse, add_video};
pub use get_random_video::{__path_get_random_video, GetRandomVideoResponse, get_random_video};
pub use get_videos::{__path_get_videos, GetVideosResponse, get_videos};
