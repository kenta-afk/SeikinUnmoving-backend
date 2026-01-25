use crate::state::GameServiceState;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use gameservice::{GameService, GameStatusResponse};

#[utoipa::path(
    get,
    path = "/game/status/{session_id}",
    tag = "game",
    params(
        ("session_id" = String, Path, description = "Game session ID")
    ),
    responses(
        (status = 200, description = "Game status retrieved", body = GameStatusResponse),
        (status = 404, description = "Session not found")
    )
)]
pub async fn get_game_status<T>(
    State(GameServiceState(service)): State<GameServiceState<T>>,
    Path(session_id): Path<String>,
) -> Result<Json<GameStatusResponse>, StatusCode>
where
    T: GameService,
{
    match service.get_game_status(&session_id) {
        Ok(response) => Ok(Json(response)),
        Err(_err) => Err(StatusCode::NOT_FOUND),
    }
}
