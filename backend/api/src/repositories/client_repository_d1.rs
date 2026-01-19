use userservice::{
    Client, ClientRepository, CreateClient, DbError, SaveClient, UserId,
};
use worker::d1::D1Database;
use std::sync::Arc;

#[derive(Clone)]
pub struct ClientRepositoryD1 {
    db: Arc<D1Database>,
}

impl ClientRepositoryD1 {
    pub fn new(db: D1Database) -> Self {
        Self { db: Arc::new(db) }
    }
}

#[async_trait::async_trait(?Send)]
impl ClientRepository for ClientRepositoryD1 {
    async fn create(&self, client: CreateClient) -> Result<(), DbError> {
        let query = self
            .db
            .prepare(
                "INSERT INTO clients (id, user_id, jti, exp, created_at) 
                 VALUES (?1, ?2, ?3, ?4, ?5)"
            )
            .bind(&[
                client.id.to_string().into(),
                client.user_id.to_string().into(),
                client.jti.to_string().into(),
                (client.exp as f64).into(),
                client.created_at.to_rfc3339().into(),
            ])
            .map_err(|e| DbError::Generic(e.to_string()))?;

        query
            .run()
            .await
            .map_err(|e| DbError::Generic(e.to_string()))?;

        Ok(())
    }

    async fn get_by_user_id(&self, user_id: UserId) -> Result<Option<Client>, DbError> {
        let query = self
            .db
            .prepare(
                "SELECT id, user_id, jti, exp, created_at 
                 FROM clients WHERE user_id = ?1"
            )
            .bind(&[user_id.to_string().into()])
            .map_err(|e| DbError::Generic(e.to_string()))?;

        let result = query
            .first::<serde_json::Value>(None)
            .await
            .map_err(|e| DbError::Generic(e.to_string()))?;

        match result {
            Some(row) => {
                let client = serde_json::from_value(row)
                    .map_err(|e| DbError::Generic(format!("Failed to deserialize client: {}", e)))?;
                Ok(Some(client))
            }
            None => Ok(None),
        }
    }

    async fn save(&self, client: SaveClient) -> Result<(), DbError> {
        let query = self
            .db
            .prepare(
                "UPDATE clients SET jti = ?1, exp = ?2 WHERE id = ?3"
            )
            .bind(&[
                client.jti.to_string().into(),
                (client.exp as f64).into(),
                client.id.to_string().into(),
            ])
            .map_err(|e| DbError::Generic(e.to_string()))?;

        query
            .run()
            .await
            .map_err(|e| DbError::Generic(e.to_string()))?;

        Ok(())
    }

    async fn delete_by_user_id(&self, user_id: UserId) -> Result<(), DbError> {
        let query = self
            .db
            .prepare("DELETE FROM clients WHERE user_id = ?1")
            .bind(&[user_id.to_string().into()])
            .map_err(|e| DbError::Generic(e.to_string()))?;

        query
            .run()
            .await
            .map_err(|e| DbError::Generic(e.to_string()))?;

        Ok(())
    }
}
