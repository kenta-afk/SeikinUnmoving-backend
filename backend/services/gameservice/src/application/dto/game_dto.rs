use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::domain::models::face_position::FacePosition;

/// ゲーム開始リクエスト
#[derive(Debug, Deserialize, ToSchema)]
pub struct StartGameRequest {
    pub user_id: String,
    #[serde(default = "default_threshold")]
    pub movement_threshold: f64,
    #[serde(default = "default_duration")]
    pub duration_seconds: i64,
}

fn default_threshold() -> f64 {
    20.0 // デフォルトの閾値（ピクセル）
}

fn default_duration() -> i64 {
    160 // デフォルトのゲーム時間（秒）= 2分40秒
}

/// ゲーム開始レスポンス
#[derive(Debug, Serialize, ToSchema)]
pub struct StartGameResponse {
    pub session_id: String,
    pub started_at: String,
    pub duration_seconds: i64,
}

/// 顔位置更新リクエスト
#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdatePositionRequest {
    pub session_id: String,
    pub position: FacePosition,
}

/// 顔位置更新レスポンス
#[derive(Debug, Serialize, ToSchema)]
pub struct UpdatePositionResponse {
    pub has_moved: bool,
    pub game_status: String,
    pub message: Option<String>,
}

/// ゲーム状態取得レスポンス
#[derive(Debug, Serialize, ToSchema)]
pub struct GameStatusResponse {
    pub session_id: String,
    pub user_id: String,
    pub status: String,
    pub started_at: String,
    pub ended_at: Option<String>,
    pub elapsed_seconds: i64,
    pub duration_seconds: i64,
}
