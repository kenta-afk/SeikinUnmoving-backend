use gameservice::{
    GameService, GameServiceImpl, StartGameRequest, StartGameResponse, UpdatePositionRequest,
    UpdatePositionResponse,
};
use worker::*;

use super::user::extractor::jwt::extract_user_id_from_jwt;

/// ゲーム開始
async fn start_game(mut req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    // JWTからユーザーIDを取得
    let user_id = match extract_user_id_from_jwt(&req) {
        Ok(id) => id,
        Err(_) => {
            return Response::error("Unauthorized", 401);
        }
    };

    let body: StartGameRequest = req.json().await?;

    // GameServiceを作成
    let game_service = GameServiceImpl::new();

    // ゲームを開始
    let mut request = body;
    request.user_id = user_id.to_string();
    
    let response = match game_service.start_game(request) {
        Ok(res) => res,
        Err(e) => {
            return Response::error(format!("Failed to start game: {}", e), 500);
        }
    };

    Response::from_json(&response)
}

/// 位置更新
async fn update_position(mut req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let body: UpdatePositionRequest = req.json().await?;

    // GameServiceを作成
    let game_service = GameServiceImpl::new();

    // 位置を更新
    let response = match game_service.update_position(body) {
        Ok(res) => res,
        Err(e) => {
            return Response::error(format!("Failed to update position: {}", e), 500);
        }
    };

    Response::from_json(&response)
}

/// ゲーム終了
async fn end_game(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let session_id = ctx.param("session_id").unwrap().to_string();

    // GameServiceを作成
    let game_service = GameServiceImpl::new();

    // ゲームを終了
    match game_service.end_game(&session_id) {
        Ok(_) => Response::ok("Game ended"),
        Err(e) => Response::error(format!("Failed to end game: {}", e), 500),
    }
}

pub fn register(router: Router<'_, ()>) -> Router<'_, ()> {
    router
        .post_async("/api/game/start", start_game)
        .post_async("/api/game/update-position", update_position)
        .post_async("/api/game/end/:session_id", end_game)
}
