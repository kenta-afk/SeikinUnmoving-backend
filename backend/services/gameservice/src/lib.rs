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

pub type ConcreteGameService = GameServiceImpl;

pub fn build_service() -> ConcreteGameService {
    GameServiceImpl::new()
}
