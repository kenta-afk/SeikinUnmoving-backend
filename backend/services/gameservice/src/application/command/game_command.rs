use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use chrono::Utc;

use crate::domain::models::game_session::{GameSession, GameSessionId, GameStatus};
use crate::domain::models::face_position::FacePosition;

/// ゲームセッション管理サービス
#[derive(Clone)]
pub struct GameSessionManager {
    sessions: Arc<Mutex<HashMap<GameSessionId, GameSession>>>,
}

impl GameSessionManager {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// 新しいゲームセッションを開始
    pub fn start_game(
        &self,
        user_id: String,
        movement_threshold: f64,
        duration_seconds: i64,
    ) -> Result<GameSession, String> {
        let session_id = format!("game_{}", uuid::Uuid::new_v4());
        let session = GameSession::new(session_id.clone(), user_id, movement_threshold, duration_seconds);

        let mut sessions = self.sessions.lock().map_err(|e| e.to_string())?;
        sessions.insert(session_id.clone(), session.clone());

        Ok(session)
    }

    /// 顔位置を更新
    pub fn update_position(
        &self,
        session_id: &str,
        position: FacePosition,
    ) -> Result<(bool, GameStatus), String> {
        let mut sessions = self.sessions.lock().map_err(|e| e.to_string())?;
        
        let session = sessions
            .get_mut(session_id)
            .ok_or_else(|| "Session not found".to_string())?;

        // ゲームが既に終了している場合
        if session.is_finished() {
            return Ok((false, session.status.clone()));
        }

        // 時間切れチェック
        if session.is_time_over() {
            session.complete_game();
            return Ok((false, GameStatus::Success));
        }

        // 顔位置を更新し、動きをチェック
        let has_moved = session.update_position(position);

        Ok((has_moved, session.status.clone()))
    }

    /// ゲームセッションを取得
    pub fn get_session(&self, session_id: &str) -> Result<GameSession, String> {
        let sessions = self.sessions.lock().map_err(|e| e.to_string())?;
        sessions
            .get(session_id)
            .cloned()
            .ok_or_else(|| "Session not found".to_string())
    }

    /// ゲームセッションを削除
    pub fn remove_session(&self, session_id: &str) -> Result<(), String> {
        let mut sessions = self.sessions.lock().map_err(|e| e.to_string())?;
        sessions.remove(session_id);
        Ok(())
    }

    /// ユーザーのアクティブなセッションを取得
    pub fn get_user_active_session(&self, user_id: &str) -> Result<Option<GameSession>, String> {
        let sessions = self.sessions.lock().map_err(|e| e.to_string())?;
        Ok(sessions
            .values()
            .find(|s| s.user_id == user_id && s.status == GameStatus::Active)
            .cloned())
    }

    /// 期限切れのセッションをクリーンアップ
    pub fn cleanup_expired_sessions(&self, max_age_seconds: i64) -> Result<usize, String> {
        let mut sessions = self.sessions.lock().map_err(|e| e.to_string())?;
        let now = Utc::now();
        
        let expired_ids: Vec<GameSessionId> = sessions
            .iter()
            .filter(|(_, session)| {
                let age = now.signed_duration_since(session.started_at);
                age.num_seconds() > max_age_seconds
            })
            .map(|(id, _)| id.clone())
            .collect();

        let count = expired_ids.len();
        for id in expired_ids {
            sessions.remove(&id);
        }

        Ok(count)
    }
}

impl Default for GameSessionManager {
    fn default() -> Self {
        Self::new()
    }
}

// uuid クレートが必要なので、簡易的なID生成を使用
mod uuid {
    use std::sync::atomic::{AtomicU64, Ordering};

    static COUNTER: AtomicU64 = AtomicU64::new(0);

    pub struct Uuid {
        value: u64,
    }

    impl Uuid {
        pub fn new_v4() -> Self {
            Self {
                value: COUNTER.fetch_add(1, Ordering::SeqCst),
            }
        }
    }

    impl std::fmt::Display for Uuid {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:016x}", self.value)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start_game() {
        let manager = GameSessionManager::new();
        let result = manager.start_game("user123".to_string(), 20.0, 30);
        assert!(result.is_ok());
        let session = result.unwrap();
        assert_eq!(session.user_id, "user123");
        assert_eq!(session.status, GameStatus::Active);
    }

    #[test]
    fn test_update_position() {
        let manager = GameSessionManager::new();
        let session = manager.start_game("user123".to_string(), 50.0, 30).unwrap();
        
        let pos1 = FacePosition::new(100.0, 100.0, 50.0, 50.0);
        let result = manager.update_position(&session.id, pos1);
        assert!(result.is_ok());
        let (has_moved, status) = result.unwrap();
        assert!(!has_moved);
        assert_eq!(status, GameStatus::Active);
    }
}
