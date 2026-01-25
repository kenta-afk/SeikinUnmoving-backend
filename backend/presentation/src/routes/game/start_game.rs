use crate::{extractors::AuthenticatedUser, state::GameServiceState};
use axum::{Json, extract::State, http::StatusCode};
use gameservice::{GameService, StartGameRequest, StartGameResponse};

#[utoipa::path(
    post,
    path = "/game/start",
    tag = "game",
    request_body = StartGameRequest,
    responses(
        (status = 200, description = "Game started successfully", body = StartGameResponse),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn start_game<T>(
    State(GameServiceState(service)): State<GameServiceState<T>>,
    AuthenticatedUser(_user_id): AuthenticatedUser,
    Json(request): Json<StartGameRequest>,
) -> Result<Json<StartGameResponse>, StatusCode>
where
    T: GameService,
{
    match service.start_game(request) {
        Ok(response) => Ok(Json(response)),
        Err(err) => {
            tracing::error!("Failed to start game: {}", err);
            Err(StatusCode::BAD_REQUEST)
        }
    }
}
