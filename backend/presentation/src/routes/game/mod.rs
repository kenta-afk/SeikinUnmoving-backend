pub mod start_game;
pub mod update_position;
pub mod get_status;
pub mod end_game;

pub use start_game::{__path_start_game, start_game};
pub use update_position::{__path_update_position, update_position};
pub use get_status::{__path_get_game_status, get_game_status};
pub use end_game::{__path_end_game, end_game};
