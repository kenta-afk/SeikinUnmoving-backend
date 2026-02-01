pub mod dto;
pub mod sample;

#[cfg(target_arch = "wasm32")]
pub mod game_repository_d1;

#[cfg(target_arch = "wasm32")]
pub use game_repository_d1::GameRepositoryD1;

#[cfg(not(target_arch = "wasm32"))]
pub mod game_repository_sqlx;

#[cfg(not(target_arch = "wasm32"))]
pub use game_repository_sqlx::GameRepositorySqlx;
