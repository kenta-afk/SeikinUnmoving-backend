use gameservice::{
    GameRepository, GameRepositoryD1, GameResult, GameStatus, StartGameRequest,
    StartGameResponse, UpdatePositionRequest, UpdatePositionResponse,
};
use serde::{Deserialize, Serialize};
use worker::*;

use crate::adapters::auth::extract_user_from_jwt;

/// ゲームセッションマネージャー（グローバル状態として保持）
static GAME_SESSIONS: once_cell::sync::Lazy<
    std::sync::Arc<std::sync::Mutex<std::collections::HashMap<String, GameSessionState>>>,
> = once_cell::sync::Lazy::new(|| std::sync::Arc::new(std::sync::Mutex::new(std::collections::HashMap::new())));

#[derive(Clone)]
struct GameSessionState {
    session_id: String,
    user_id: String,
    started_at: chrono::DateTime<chrono::Utc>,
    duration_seconds: i64,
    status: GameStatus,
}

/// ゲーム開始
async fn start_game(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    // JWTからユーザー情報を取得
    let user = match extract_user_from_jwt(&req, &ctx.env) {
        Ok(user) => user,
        Err(_) => {
            return Response::error("Unauthorized", 401);
        }
    };

    let body: StartGameRequest = req.json().await?;

    // セッションIDを生成
    let session_id = format!("game_{}", uuid::Uuid::new_v4());
    let started_at = chrono::Utc::now();

    let session_state = GameSessionState {
        session_id: session_id.clone(),
        user_id: user.user_id.clone(),
        started_at,
        duration_seconds: body.duration_seconds,
        status: GameStatus::Active,
    };

    // セッションを保存
    {
        let mut sessions = GAME_SESSIONS.lock().unwrap();
        sessions.insert(session_id.clone(), session_state);
    }

    let response = StartGameResponse {
        session_id,
        started_at: started_at.to_rfc3339(),
        duration_seconds: body.duration_seconds,
    };

    Response::from_json(&response)
}

/// 位置更新
async fn update_position(mut req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let body: UpdatePositionRequest = req.json().await?;

    let mut sessions = GAME_SESSIONS.lock().unwrap();
    let session = match sessions.get_mut(&body.session_id) {
        Some(s) => s,
        None => return Response::error("Session not found", 404),
    };

    // ゲームが既に終了している場合
    if session.status != GameStatus::Active {
        let response = UpdatePositionResponse {
            has_moved: false,
            game_status: format!("{:?}", session.status).to_lowercase(),
            message: Some("Game already ended".to_string()),
        };
        return Response::from_json(&response);
    }

    // 時間切れチェック
    let elapsed = chrono::Utc::now()
        .signed_duration_since(session.started_at)
        .num_seconds();
    
    if elapsed >= session.duration_seconds {
        session.status = GameStatus::Success;
        let response = UpdatePositionResponse {
            has_moved: false,
            game_status: "success".to_string(),
            message: Some("Time's up! You won!".to_string()),
        };
        return Response::from_json(&response);
    }

    // 簡易的な動き検出（実際の実装では顔位置を比較）
    let has_moved = false; // TODO: 実際の動き検出ロジック

    let response = UpdatePositionResponse {
        has_moved,
        game_status: "active".to_string(),
        message: None,
    };

    Response::from_json(&response)
}

/// ゲーム終了
async fn end_game(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let session_id = ctx.param("session_id").unwrap().to_string();

    // セッションを取得
    let session = {
        let sessions = GAME_SESSIONS.lock().unwrap();
        sessions.get(&session_id).cloned()
    };

    let session = match session {
        Some(s) => s,
        None => return Response::error("Session not found", 404),
    };

    // 時間切れチェック
    let elapsed = chrono::Utc::now()
        .signed_duration_since(session.started_at)
        .num_seconds();
    
    let is_clear = if elapsed >= session.duration_seconds {
        true
    } else {
        match session.status {
            GameStatus::Success => true,
            _ => false,
        }
    };

    // D1データベースに保存
    let db = ctx.env.d1("DB")?;
    let repo = GameRepositoryD1::new(db);
    
    let game_result = GameResult::new(
        format!("game_{}", uuid::Uuid::new_v4()),
        session.user_id.clone(),
        is_clear,
    );

    repo.save_game_result(game_result)
        .await
        .map_err(|e| worker::Error::RustError(e))?;

    // セッションを削除
    {
        let mut sessions = GAME_SESSIONS.lock().unwrap();
        sessions.remove(&session_id);
    }

    Response::ok("Game ended")
}

pub fn register(router: Router<'_, ()>) -> Router<'_, ()> {
    router
        .post_async("/api/game/start", start_game)
        .post_async("/api/game/update-position", update_position)
        .post_async("/api/game/end/:session_id", end_game)
}
