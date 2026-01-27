use crate::{
    extractors::AuthenticatedUser,
    state::{GameServiceState, UserServiceState},
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use gameservice::GameService;
use serde::Deserialize;
use userservice::UserService;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct EndGameRequest {
    pub seikin_similarity: Option<f64>,
}

//endpoint to end a game session
#[utoipa::path(
    post,
    path = "/game/end/{session_id}",
    tag = "game",
    params(
        ("session_id" = String, Path, description = "Game session ID")
    ),
    request_body = EndGameRequest,
    responses(
        (status = 200, description = "Game ended successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Session not found")
    )
)]
pub async fn end_game<T, U>(
    State(GameServiceState(game_service)): State<GameServiceState<T>>,
    State(UserServiceState(user_service)): State<UserServiceState<U>>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    Path(session_id): Path<String>,
    Json(request): Json<EndGameRequest>,
) -> Result<StatusCode, StatusCode>
where
    T: GameService,
    U: UserService,
{
    println!(
        "end_game called: user_id={}, session_id={}, seikin_similarity={:?}",
        user_id, session_id, request.seikin_similarity
    );

    // ゲームセッションを終了
    match game_service.end_game(&session_id) {
        Ok(_) => {
            // セイキン類似度が提供されている場合は更新（成功時のみ送信される）
            if let Some(similarity) = request.seikin_similarity {
                println!(
                    "Updating seikin_similarity for user {} to {}",
                    user_id, similarity
                );
                if let Err(e) = user_service
                    .update_seikin_similarity(user_id, similarity)
                    .await
                {
                    eprintln!(
                        "Failed to update seikin_similarity for user {}: {:?}",
                        user_id, e
                    );
                } else {
                    println!(
                        "Successfully updated seikin_similarity for user {}",
                        user_id
                    );
                }
            } else {
                println!("No seikin_similarity provided in request");
            }
            Ok(StatusCode::OK)
        }
        Err(_err) => Err(StatusCode::NOT_FOUND),
    }
}
