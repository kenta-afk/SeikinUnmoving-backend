#[cfg(target_arch = "wasm32")]
use chrono::{DateTime, Utc};
#[cfg(target_arch = "wasm32")]
use std::sync::Arc;
#[cfg(target_arch = "wasm32")]
use worker::d1::D1Database;

#[cfg(target_arch = "wasm32")]
use crate::domain::game_repository::{GameRepository, GameResult};

/// D1データベースを使用したゲームリポジトリ実装
#[cfg(target_arch = "wasm32")]
#[derive(Clone)]
pub struct GameRepositoryD1 {
    db: Arc<D1Database>,
}

#[cfg(target_arch = "wasm32")]
impl GameRepositoryD1 {
    pub fn new(db: D1Database) -> Self {
        Self { db: Arc::new(db) }
    }
}

#[cfg(target_arch = "wasm32")]
#[async_trait::async_trait(?Send)]
impl GameRepository for GameRepositoryD1 {
    async fn save_game_result(&self, result: GameResult) -> Result<(), String> {
        let is_clear_value = if result.is_clear { 1 } else { 0 };

        self.db
            .prepare(
                "INSERT INTO games (id, user_id, is_clear, created_at) VALUES (?1, ?2, ?3, ?4)",
            )
            .bind(&[
                result.id.into(),
                result.user_id.into(),
                is_clear_value.into(),
                result.created_at.to_rfc3339().into(),
            ])
            .map_err(|e| format!("Failed to bind parameters: {:?}", e))?
            .run()
            .await
            .map_err(|e| format!("Failed to save game result: {:?}", e))?;

        Ok(())
    }

    async fn update_game_result(&self, game_id: &str, is_clear: bool) -> Result<(), String> {
        let is_clear_value = if is_clear { 1 } else { 0 };

        self.db
            .prepare("UPDATE games SET is_clear = ?1 WHERE id = ?2")
            .bind(&[is_clear_value.into(), game_id.into()])
            .map_err(|e| format!("Failed to bind parameters: {:?}", e))?
            .run()
            .await
            .map_err(|e| format!("Failed to update game result: {:?}", e))?;

        Ok(())
    }

    async fn get_user_game_results(&self, user_id: &str) -> Result<Vec<GameResult>, String> {
        let results = self
            .db
            .prepare("SELECT id, user_id, is_clear, created_at FROM games WHERE user_id = ?1 ORDER BY created_at DESC")
            .bind(&[user_id.into()])
            .map_err(|e| format!("Failed to bind parameters: {:?}", e))?
            .all()
            .await
            .map_err(|e| format!("Failed to get game results: {:?}", e))?;

        let results_data = results
            .results::<serde_json::Value>()
            .map_err(|e| format!("Failed to parse results: {:?}", e))?;

        let mut game_results = Vec::new();
        for row in results_data {
            let id = row["id"].as_str().ok_or("Missing id")?.to_string();
            let user_id = row["user_id"]
                .as_str()
                .ok_or("Missing user_id")?
                .to_string();
            let is_clear = row["is_clear"].as_i64().ok_or("Missing is_clear")? != 0;
            let created_at_str = row["created_at"].as_str().ok_or("Missing created_at")?;
            let created_at = DateTime::parse_from_rfc3339(created_at_str)
                .map_err(|e| format!("Failed to parse created_at: {:?}", e))?
                .with_timezone(&Utc);

            game_results.push(GameResult {
                id,
                user_id,
                is_clear,
                created_at,
            });
        }

        Ok(game_results)
    }

    async fn get_clear_count(&self, user_id: &str) -> Result<i64, String> {
        let result = self
            .db
            .prepare("SELECT COUNT(*) as count FROM games WHERE user_id = ?1 AND is_clear = 1")
            .bind(&[user_id.into()])
            .map_err(|e| format!("Failed to bind parameters: {:?}", e))?
            .first::<serde_json::Value>(None)
            .await
            .map_err(|e| format!("Failed to get clear count: {:?}", e))?;

        let count = result
            .ok_or("No result")?
            .get("count")
            .and_then(|v| v.as_i64())
            .ok_or("Failed to get count")?;

        Ok(count)
    }
}
