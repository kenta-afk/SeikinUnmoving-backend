use chrono::Utc;

use super::{
    command::game_command::GameSessionManager,
    dto::game_dto::{
        GameStatusResponse, StartGameRequest, StartGameResponse, UpdatePositionRequest,
        UpdatePositionResponse,
    },
};
use crate::domain::models::game_session::GameStatus;

/// ゲームサービストレイト
pub trait GameService: Clone + Send + Sync + 'static {
    fn start_game(&self, request: StartGameRequest) -> Result<StartGameResponse, String>;
    fn update_position(
        &self,
        request: UpdatePositionRequest,
    ) -> Result<UpdatePositionResponse, String>;
    fn get_game_status(&self, session_id: &str) -> Result<GameStatusResponse, String>;
    fn end_game(&self, session_id: &str) -> Result<(), String>;
    fn cleanup_expired_sessions(&self) -> Result<usize, String>;
}

/// ゲームサービス実装
#[derive(Clone)]
pub struct GameServiceImpl {
    session_manager: GameSessionManager,
}

impl GameServiceImpl {
    pub fn new() -> Self {
        Self {
            session_manager: GameSessionManager::new(),
        }
    }
}

impl GameService for GameServiceImpl {
    /// ゲームを開始
    fn start_game(&self, request: StartGameRequest) -> Result<StartGameResponse, String> {
        // 既にアクティブなセッションがある場合は自動終了
        if let Some(session) = self
            .session_manager
            .get_user_active_session(&request.user_id)?
        {
            // 既存セッションを削除
            let _ = self.session_manager.remove_session(&session.id);
        }

        let session = self.session_manager.start_game(
            request.user_id,
            request.movement_threshold,
            request.duration_seconds,
        )?;

        Ok(StartGameResponse {
            session_id: session.id,
            started_at: session.started_at.to_rfc3339(),
            duration_seconds: session.duration_seconds,
        })
    }

    /// 顔位置を更新
    fn update_position(
        &self,
        request: UpdatePositionRequest,
    ) -> Result<UpdatePositionResponse, String> {
        let (has_moved, status) = self
            .session_manager
            .update_position(&request.session_id, request.position)?;

        let (game_status, message) = match status {
            GameStatus::Active => ("active".to_string(), None),
            GameStatus::Failed => (
                "failed".to_string(),
                Some("You moved! Game over.".to_string()),
            ),
            GameStatus::Success => (
                "success".to_string(),
                Some("Congratulations! You stayed still!".to_string()),
            ),
        };

        Ok(UpdatePositionResponse {
            has_moved,
            game_status,
            message,
        })
    }

    /// ゲームの状態を取得
    fn get_game_status(&self, session_id: &str) -> Result<GameStatusResponse, String> {
        let session = self.session_manager.get_session(session_id)?;

        let elapsed = Utc::now()
            .signed_duration_since(session.started_at)
            .num_seconds();

        let status_str = match session.status {
            GameStatus::Active => "active",
            GameStatus::Failed => "failed",
            GameStatus::Success => "success",
        };

        Ok(GameStatusResponse {
            session_id: session.id,
            user_id: session.user_id,
            status: status_str.to_string(),
            started_at: session.started_at.to_rfc3339(),
            ended_at: session.ended_at.map(|dt| dt.to_rfc3339()),
            elapsed_seconds: elapsed,
            duration_seconds: session.duration_seconds,
        })
    }

    /// ゲームセッションを終了
    fn end_game(&self, session_id: &str) -> Result<(), String> {
        self.session_manager.remove_session(session_id)
    }

    /// 期限切れセッションをクリーンアップ
    fn cleanup_expired_sessions(&self) -> Result<usize, String> {
        // 1時間以上経過したセッションを削除
        self.session_manager.cleanup_expired_sessions(3600)
    }
}
