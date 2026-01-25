mod application;
mod domain;
mod infrastructure;

// 公開API
pub use application::{
    command::game_command::GameSessionManager,
    dto::game_dto::{
        GameStatusResponse, StartGameRequest, StartGameResponse, UpdatePositionRequest,
        UpdatePositionResponse,
    },
    game_service::{GameService, GameServiceImpl},
};
pub use domain::models::{
    face_position::FacePosition,
    game_session::{GameSession, GameStatus},
};
pub use domain::game_repository::{GameRepository, GameResult};

#[cfg(target_arch = "wasm32")]
pub use infrastructure::repositories::GameRepositoryD1;

pub type ConcreteGameService = GameServiceImpl;

pub fn build_service() -> ConcreteGameService {
    GameServiceImpl::new()
}
