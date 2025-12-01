use sqlx::SqlitePool;

use crate::domain::{
    models::{client::Client, error::DbError},
    repositories::client_repository::ClientRepository,
};

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
    async fn save(&self, client: Client) -> Result<(), DbError> {
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
}
