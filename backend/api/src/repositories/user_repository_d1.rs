use userservice::{
    CreateUser, DbError, User, UserId, UserRepository,
};
use worker::d1::D1Database;
use std::sync::Arc;

#[derive(Clone)]
pub struct UserRepositoryD1 {
    db: Arc<D1Database>,
}

impl UserRepositoryD1 {
    pub fn new(db: D1Database) -> Self {
        Self { db: Arc::new(db) }
    }
}

#[async_trait::async_trait(?Send)]
impl UserRepository for UserRepositoryD1 {
    async fn create(&self, user: CreateUser) -> Result<(), DbError> {
        let query = self
            .db
            .prepare(
                "INSERT INTO users (id, name, email, password, seikin_similarity, created_at, updated_at) 
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)"
            )
            .bind(&[
                user.id.to_string().into(),
                user.name.into(),
                user.email.into(),
                user.password.into(),
                user.seikin_similarity.into(),
                user.created_at.to_rfc3339().into(),
                user.updated_at.to_rfc3339().into(),
            ])
            .map_err(|e| DbError::Generic(e.to_string()))?;

        query
            .run()
            .await
            .map_err(|e| DbError::Generic(e.to_string()))?;

        Ok(())
    }

    async fn get_by_email(&self, email: &str) -> Result<Option<User>, DbError> {
        let query = self
            .db
            .prepare(
                "SELECT id, name, email, password, seikin_similarity, created_at, updated_at 
                 FROM users WHERE email = ?1"
            )
            .bind(&[email.into()])
            .map_err(|e| DbError::Generic(e.to_string()))?;

        let result = query
            .first::<serde_json::Value>(None)
            .await
            .map_err(|e| DbError::Generic(e.to_string()))?;

        match result {
            Some(row) => {
                // JSONからUserを構築
                let user = serde_json::from_value(row)
                    .map_err(|e| DbError::Generic(format!("Failed to deserialize user: {}", e)))?;
                Ok(Some(user))
            }
            None => Ok(None),
        }
    }

    async fn get_by_id(&self, id: UserId) -> Result<Option<User>, DbError> {
        let query = self
            .db
            .prepare(
                "SELECT id, name, email, password, seikin_similarity, created_at, updated_at 
                 FROM users WHERE id = ?1"
            )
            .bind(&[id.to_string().into()])
            .map_err(|e| DbError::Generic(e.to_string()))?;

        let result = query
            .first::<serde_json::Value>(None)
            .await
            .map_err(|e| DbError::Generic(e.to_string()))?;

        match result {
            Some(row) => {
                let user = serde_json::from_value(row)
                    .map_err(|e| DbError::Generic(format!("Failed to deserialize user: {}", e)))?;
                Ok(Some(user))
            }
            None => Ok(None),
        }
    }
}
