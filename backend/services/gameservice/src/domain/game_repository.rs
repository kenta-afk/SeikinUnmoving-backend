use chrono::{DateTime, Utc};

/// ゲーム結果
#[derive(Debug, Clone)]
pub struct GameResult {
    pub id: String,
    pub user_id: String,
    pub is_clear: bool,
    pub created_at: DateTime<Utc>,
}

impl GameResult {
    pub fn new(id: String, user_id: String, is_clear: bool) -> Self {
        Self {
            id,
            user_id,
            is_clear,
            created_at: Utc::now(),
        }
    }
}

/// ゲームリポジトリトレイト
#[cfg(not(target_arch = "wasm32"))]
#[async_trait::async_trait]
pub trait GameRepository: Send + Sync {
    /// ゲーム結果を保存
    async fn save_game_result(&self, result: GameResult) -> Result<(), String>;

    /// ユーザーのゲーム結果を取得
    async fn get_user_game_results(&self, user_id: &str) -> Result<Vec<GameResult>, String>;

    /// クリアしたゲーム数を取得
    async fn get_clear_count(&self, user_id: &str) -> Result<i64, String>;
}

#[cfg(target_arch = "wasm32")]
#[async_trait::async_trait(?Send)]
pub trait GameRepository {
    /// ゲーム結果を保存
    async fn save_game_result(&self, result: GameResult) -> Result<(), String>;

    /// ユーザーのゲーム結果を取得
    async fn get_user_game_results(&self, user_id: &str) -> Result<Vec<GameResult>, String>;

    /// クリアしたゲーム数を取得
    async fn get_clear_count(&self, user_id: &str) -> Result<i64, String>;
}
