use crate::domain::{
    models::{client::Client, error::DbError, id::UserId},
    repositories::client::{
        client_repository::ClientRepository, create_client::CreateClient, save_client::SaveClient,
    },
};

// ローカル開発環境用（WASM以外）
#[cfg(not(target_arch = "wasm32"))]
use crate::domain::models::id::ClientId;
#[cfg(not(target_arch = "wasm32"))]
use sqlx::SqlitePool;
#[cfg(not(target_arch = "wasm32"))]
use std::str::FromStr;
#[cfg(not(target_arch = "wasm32"))]
use uuid::Uuid;

// WASM環境用の実装
#[cfg(target_arch = "wasm32")]
#[derive(Clone, Default)]
pub struct ClientRepositoryImpl {}

#[cfg(target_arch = "wasm32")]
impl ClientRepositoryImpl {
    pub fn new() -> Self {
        Self {}
    }
}

#[cfg(target_arch = "wasm32")]
#[async_trait::async_trait(?Send)]
impl ClientRepository for ClientRepositoryImpl {
    async fn create(&self, _client: CreateClient) -> Result<(), DbError> {
        Err(DbError::Generic("Not implemented for WASM".to_string()))
    }
    async fn get_by_user_id(&self, _user_id: UserId) -> Result<Option<Client>, DbError> {
        Err(DbError::Generic("Not implemented for WASM".to_string()))
    }
    async fn save(&self, _client: SaveClient) -> Result<(), DbError> {
        Err(DbError::Generic("Not implemented for WASM".to_string()))
    }
    async fn delete_by_user_id(&self, _user_id: UserId) -> Result<(), DbError> {
        Err(DbError::Generic("Not implemented for WASM".to_string()))
    }
}

// ローカル開発環境用の実装（SQLXを使用）
#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone)]
pub struct ClientRepositoryImpl {
    pool: SqlitePool,
}

#[cfg(not(target_arch = "wasm32"))]
impl ClientRepositoryImpl {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[async_trait::async_trait]
impl ClientRepository for ClientRepositoryImpl {
    async fn create(&self, client: CreateClient) -> Result<(), DbError> {
        let id = client.id.to_string();
        let user_id = client.user_id.to_string();
        let jti = client.jti.to_string();
        let exp = client.exp;
        let created_at = client.created_at;

        sqlx::query!(
            r#"
            INSERT INTO clients (id, user_id, jti, exp, created_at)
            VALUES (?, ?, ?, ?, ?)
            "#,
            id,
            user_id,
            jti,
            exp,
            created_at
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn get_by_user_id(&self, user_id: UserId) -> Result<Option<Client>, DbError> {
        let user_id_str = user_id.to_string();
        let result = sqlx::query!(
            r#"
            SELECT id, user_id, jti, exp, created_at
            FROM clients
            WHERE user_id = ?
            "#,
            user_id_str
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.map(|row| {
            let id = ClientId::from_str(&row.id).expect("Invalid UUID");
            let user_id = UserId::from_str(&row.user_id).expect("Invalid UUID");
            let jti = Uuid::parse_str(&row.jti).expect("Invalid UUID");
            let created_at = chrono::DateTime::parse_from_rfc3339(&row.created_at)
                .expect("Invalid datetime")
                .with_timezone(&chrono::Utc);
            Client::from_db(id, user_id, jti, row.exp, created_at)
        }))
    }

    async fn save(&self, client: SaveClient) -> Result<(), DbError> {
        let jti = client.jti.to_string();
        let exp = client.exp;
        let id = client.id.to_string();

        sqlx::query!(
            r#"
            UPDATE clients
            SET jti = ?, exp = ?
            WHERE id = ?
            "#,
            jti,
            exp,
            id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn delete_by_user_id(&self, user_id: UserId) -> Result<(), DbError> {
        let user_id = user_id.to_string();

        sqlx::query!(
            r#"
            DELETE FROM clients
            WHERE user_id = ?
            "#,
            user_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
