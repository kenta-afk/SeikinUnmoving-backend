use sqlx::SqlitePool;

use crate::{
    domain::{
        models::{
            client::Client,
            error::DbError,
            id::{ClientId, UserId},
        },
        repositories::client::{
            client_repository::ClientRepository, create_client::CreateClient,
            save_client::SaveClient,
        },
    },
    infrastructure::repositories::client::db_client::DbClient,
};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone)]
pub struct ClientRepositoryImpl {
    pub pool: SqlitePool,
}

#[allow(dead_code)]
impl ClientRepositoryImpl {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl ClientRepository for ClientRepositoryImpl {
    async fn create(&self, client: CreateClient) -> Result<(), DbError> {
        sqlx::query!(
            r#"
            INSERT INTO clients (id, user_id, jti, exp, created_at)
            VALUES (?1, ?2, ?3, ?4, ?5)
            "#,
            client.id,
            client.user_id,
            client.jti,
            client.exp,
            client.created_at,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
    async fn get_by_user_id(&self, user_id: UserId) -> Result<Option<Client>, DbError> {
        let record = sqlx::query_as!(
            DbClient,
            r#"
            SELECT 
                id as "id: ClientId", 
                user_id as "user_id: UserId", 
                jti as "jti: Uuid", 
                exp as "exp: DateTime<Utc>", 
                created_at as "created_at: DateTime<Utc>"
            FROM clients
            WHERE user_id = ?1
            "#,
            user_id,
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(record.map(Into::into))
    }
    async fn save(&self, client: SaveClient) -> Result<(), DbError> {
        sqlx::query!(
            r#"
            UPDATE clients
            SET jti = ?1, exp = ?2
            WHERE id = ?3
            "#,
            client.jti,
            client.exp,
            client.id,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
