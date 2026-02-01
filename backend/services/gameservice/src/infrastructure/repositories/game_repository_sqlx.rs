#[cfg(not(target_arch = "wasm32"))]
use chrono::{DateTime, Utc};
#[cfg(not(target_arch = "wasm32"))]
use sqlx::SqlitePool;

#[cfg(not(target_arch = "wasm32"))]
use crate::domain::game_repository::{GameRepository, GameResult};

/// SQLiteデータベースを使用したゲームリポジトリ実装
#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone)]
pub struct GameRepositorySqlx {
    pool: SqlitePool,
}

#[cfg(not(target_arch = "wasm32"))]
impl GameRepositorySqlx {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[async_trait::async_trait]
impl GameRepository for GameRepositorySqlx {
    async fn save_game_result(&self, result: GameResult) -> Result<(), String> {
        let is_clear = if result.is_clear { 1 } else { 0 };
        let created_at = result.created_at.to_rfc3339();

        sqlx::query!(
            r#"
            INSERT INTO games (id, user_id, is_clear, created_at)
            VALUES (?, ?, ?, ?)
            "#,
            result.id,
            result.user_id,
            is_clear,
            created_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Failed to save game result: {:?}", e))?;

        Ok(())
    }

    async fn update_game_result(&self, game_id: &str, is_clear: bool) -> Result<(), String> {
        let is_clear_value = if is_clear { 1 } else { 0 };

        sqlx::query!(
            r#"
            UPDATE games
            SET is_clear = ?
            WHERE id = ?
            "#,
            is_clear_value,
            game_id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Failed to update game result: {:?}", e))?;

        Ok(())
    }

    async fn get_user_game_results(&self, user_id: &str) -> Result<Vec<GameResult>, String> {
        let records = sqlx::query!(
            r#"
            SELECT id, user_id, is_clear, created_at
            FROM games
            WHERE user_id = ?
            ORDER BY created_at DESC
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Failed to get game results: {:?}", e))?;

        let mut results = Vec::new();
        for record in records {
            let created_at = DateTime::parse_from_rfc3339(&record.created_at)
                .map_err(|e| format!("Failed to parse created_at: {:?}", e))?
                .with_timezone(&Utc);

            results.push(GameResult {
                id: record.id,
                user_id: record.user_id,
                is_clear: record.is_clear != 0,
                created_at,
            });
        }

        Ok(results)
    }

    async fn get_clear_count(&self, user_id: &str) -> Result<i64, String> {
        let record = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM games
            WHERE user_id = ? AND is_clear = 1
            "#,
            user_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| format!("Failed to get clear count: {:?}", e))?;

        Ok(record.count as i64)
    }
}
