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
pub use domain::{
    game_repository::{GameRepository, GameResult},
    models::{
        face_position::FacePosition,
        game_session::{GameSession, GameStatus},
    },
};

#[cfg(target_arch = "wasm32")]
pub use infrastructure::repositories::GameRepositoryD1;

#[cfg(not(target_arch = "wasm32"))]
pub use infrastructure::repositories::GameRepositorySqlx;

#[cfg(not(target_arch = "wasm32"))]
pub type ConcreteGameService = GameServiceImpl<GameRepositorySqlx>;

#[cfg(target_arch = "wasm32")]
pub type ConcreteGameService = GameServiceImpl<GameRepositoryD1>;

#[cfg(not(target_arch = "wasm32"))]
pub fn build_service(database_url: &str) -> Result<ConcreteGameService, String> {
    let pool = sqlx::SqlitePool::connect_lazy(database_url)
        .map_err(|e| format!("Failed to connect to database: {:?}", e))?;
    let repository = GameRepositorySqlx::new(pool);
    Ok(GameServiceImpl::new(repository))
}

#[cfg(target_arch = "wasm32")]
pub fn build_service_with_d1(db: worker::d1::D1Database) -> ConcreteGameService {
    let repository = GameRepositoryD1::new(db);
    GameServiceImpl::new(repository)
}
