use chrono::{DateTime, Utc};
use sqlx::SqlitePool;

use crate::{
    domain::{
        models::{error::DbError, id::UserId, user::User},
        repositories::user_repository::UserRepository,
    },
    infrastructure::repositories::dto::user::db_user::DbUser,
};

pub struct UserRepositoryImpl {
    pub pool: SqlitePool,
}

#[allow(dead_code)]
impl UserRepositoryImpl {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn save(&self, user: User) -> Result<(), DbError> {
        sqlx::query!(
            r#"
            INSERT INTO users (id, name, email, password, seikin_similarity, created_at, updated_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
            "#,
            user.id,
            user.name,
            user.email,
            user.password,
            user.seikin_similarity,
            user.created_at,
            user.updated_at,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
    async fn get_by_id(&self, user_id: UserId) -> Result<Option<User>, DbError> {
        let record = sqlx::query_as!(
            DbUser,
            r#"
            SELECT 
                id as "id: UserId", 
                name, 
                email, 
                password, 
                seikin_similarity,
                created_at as "created_at: DateTime<Utc>", 
                updated_at as "updated_at: DateTime<Utc>"
            FROM users
            WHERE id = ?1
            "#,
            user_id,
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(record.map(Into::into))
    }
}
