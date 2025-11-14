use sqlx::SqlitePool;

use crate::domain::{
    models::{error::DbError, user::User},
    user_repository::UserRepository,
};

#[allow(dead_code)]
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
            INSERT INTO users (id, username, email, password, created_at, updated_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)
            "#,
            user.id,
            user.name,
            user.email,
            user.password,
            user.created_at,
            user.updated_at,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
