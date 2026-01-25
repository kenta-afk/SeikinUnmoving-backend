use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::face_position::FacePosition;

/// ゲームセッションのID
pub type GameSessionId = String;

/// ゲームセッションの状態
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GameStatus {
    /// ゲーム進行中
    Active,
    /// ゲーム終了（負け - 動いた）
    Failed,
    /// ゲーム終了（時間切れ - 成功）
    Success,
}

/// ゲームセッション
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameSession {
    /// セッションID
    pub id: GameSessionId,
    /// ユーザーID
    pub user_id: String,
    /// ゲーム状態
    pub status: GameStatus,
    /// 開始時刻
    pub started_at: DateTime<Utc>,
    /// 終了時刻
    pub ended_at: Option<DateTime<Utc>>,
    /// 前回の顔位置
    pub last_position: Option<FacePosition>,
    /// 動き検出の閾値
    pub movement_threshold: f64,
    /// ゲーム時間（秒）
    pub duration_seconds: i64,
}

impl GameSession {
    /// 新しいゲームセッションを作成
    pub fn new(
        id: GameSessionId,
        user_id: String,
        movement_threshold: f64,
        duration_seconds: i64,
    ) -> Self {
        Self {
            id,
            user_id,
            status: GameStatus::Active,
            started_at: Utc::now(),
            ended_at: None,
            last_position: None,
            movement_threshold,
            duration_seconds,
        }
    }

    /// 顔位置を更新し、動きがあったかチェック
    pub fn update_position(&mut self, new_position: FacePosition) -> bool {
        if self.status != GameStatus::Active {
            return false;
        }

        let has_moved = if let Some(ref last_pos) = self.last_position {
            last_pos.has_moved(&new_position, self.movement_threshold)
        } else {
            false // 初回は動きなしと判定
        };

        self.last_position = Some(new_position);

        if has_moved {
            self.fail_game();
        }

        has_moved
    }

    /// ゲームを失敗状態にする
    pub fn fail_game(&mut self) {
        if self.status == GameStatus::Active {
            self.status = GameStatus::Failed;
            self.ended_at = Some(Utc::now());
        }
    }

    /// ゲームを成功状態にする
    pub fn complete_game(&mut self) {
        if self.status == GameStatus::Active {
            self.status = GameStatus::Success;
            self.ended_at = Some(Utc::now());
        }
    }

    /// ゲームが終了しているかチェック
    pub fn is_finished(&self) -> bool {
        self.status != GameStatus::Active
    }

    /// ゲームの経過時間が制限時間を超えたかチェック
    pub fn is_time_over(&self) -> bool {
        let elapsed = Utc::now().signed_duration_since(self.started_at);
        elapsed.num_seconds() >= self.duration_seconds
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_game_session() {
        let session =
            GameSession::new("test-session".to_string(), "user-123".to_string(), 10.0, 30);
        assert_eq!(session.status, GameStatus::Active);
        assert!(session.last_position.is_none());
    }

    #[test]
    fn test_update_position_no_movement() {
        let mut session =
            GameSession::new("test-session".to_string(), "user-123".to_string(), 50.0, 30);

        let pos1 = FacePosition::new(100.0, 100.0, 50.0, 50.0);
        let pos2 = FacePosition::new(102.0, 102.0, 50.0, 50.0);

        assert!(!session.update_position(pos1));
        assert!(!session.update_position(pos2));
        assert_eq!(session.status, GameStatus::Active);
    }

    #[test]
    fn test_update_position_with_movement() {
        let mut session =
            GameSession::new("test-session".to_string(), "user-123".to_string(), 10.0, 30);

        let pos1 = FacePosition::new(100.0, 100.0, 50.0, 50.0);
        let pos2 = FacePosition::new(200.0, 200.0, 50.0, 50.0);

        assert!(!session.update_position(pos1));
        assert!(session.update_position(pos2));
        assert_eq!(session.status, GameStatus::Failed);
        assert!(session.ended_at.is_some());
    }
}
