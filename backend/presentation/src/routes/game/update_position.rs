use crate::state::GameServiceState;
use axum::{extract::State, http::StatusCode, Json};
use gameservice::{GameService, UpdatePositionRequest, UpdatePositionResponse};

#[utoipa::path(
    post,
    path = "/game/update-position",
    tag = "game",
    request_body = UpdatePositionRequest,
    responses(
        (status = 200, description = "Position updated successfully", body = UpdatePositionResponse),
        (status = 400, description = "Bad request")
    )
)]
pub async fn update_position<T>(
    State(GameServiceState(service)): State<GameServiceState<T>>,
    Json(request): Json<UpdatePositionRequest>,
) -> Result<Json<UpdatePositionResponse>, StatusCode>
where
    T: GameService,
{
    match service.update_position(request) {
        Ok(response) => Ok(Json(response)),
        Err(_err) => Err(StatusCode::BAD_REQUEST),
    }
}
