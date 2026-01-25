use crate::{extractors::AuthenticatedUser, state::GameServiceState};
use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use gameservice::GameService;

#[utoipa::path(
    post,
    path = "/game/end/{session_id}",
    tag = "game",
    params(
        ("session_id" = String, Path, description = "Game session ID")
    ),
    responses(
        (status = 200, description = "Game ended successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Session not found")
    )
)]
pub async fn end_game<T>(
    State(GameServiceState(service)): State<GameServiceState<T>>,
    AuthenticatedUser(_user_id): AuthenticatedUser,
    Path(session_id): Path<String>,
) -> Result<StatusCode, StatusCode>
where
    T: GameService,
{
    match service.end_game(&session_id) {
        Ok(_) => Ok(StatusCode::OK),
        Err(_err) => Err(StatusCode::NOT_FOUND),
    }
}
