use crate::domain::{
    models::{error::DbError, id::UserId, user::User},
    repositories::user::{create_user::CreateUser, user_repository::UserRepository},
};

// ローカル開発環境用（WASM以外）
#[cfg(not(target_arch = "wasm32"))]
use sqlx::SqlitePool;
#[cfg(not(target_arch = "wasm32"))]
use std::str::FromStr;

// WASM環境用の実装
#[cfg(target_arch = "wasm32")]
#[derive(Clone, Default)]
pub struct UserRepositoryImpl {}

#[cfg(target_arch = "wasm32")]
impl UserRepositoryImpl {
    pub fn new() -> Self {
        Self {}
    }
}

#[cfg(target_arch = "wasm32")]
#[async_trait::async_trait(?Send)]
impl UserRepository for UserRepositoryImpl {
    async fn create(&self, _user: CreateUser) -> Result<(), DbError> {
        Err(DbError::Generic("Not implemented for WASM".to_string()))
    }
    async fn get_by_email(&self, _email: &str) -> Result<Option<User>, DbError> {
        Err(DbError::Generic("Not implemented for WASM".to_string()))
    }
    async fn get_by_id(&self, _id: UserId) -> Result<Option<User>, DbError> {
        Err(DbError::Generic("Not implemented for WASM".to_string()))
    }
}

// ローカル開発環境用の実装（SQLXを使用）
#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone)]
pub struct UserRepositoryImpl {
    pool: SqlitePool,
}

#[cfg(not(target_arch = "wasm32"))]
impl UserRepositoryImpl {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[async_trait::async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn create(&self, user: CreateUser) -> Result<(), DbError> {
        let id = user.id.to_string();
        let name = user.name;
        let email = user.email;
        let password = user.password;
        let seikin_similarity = user.seikin_similarity;
        let created_at = user.created_at;
        let updated_at = user.updated_at;

        sqlx::query!(
            r#"
            INSERT INTO users (id, name, email, password, seikin_similarity, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
            id,
            name,
            email,
            password,
            seikin_similarity,
            created_at,
            updated_at
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn get_by_email(&self, email: &str) -> Result<Option<User>, DbError> {
        let result = sqlx::query!(
            r#"
            SELECT id, name, email, password, seikin_similarity, created_at, updated_at
            FROM users
            WHERE email = ?
            "#,
            email
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.map(|row| {
            let id = UserId::from_str(&row.id).expect("Invalid UUID");
            User::from_db(
                id,
                row.name,
                row.email,
                row.password,
                row.seikin_similarity,
                chrono::DateTime::parse_from_rfc3339(&row.created_at)
                    .expect("Invalid datetime")
                    .with_timezone(&chrono::Utc),
                chrono::DateTime::parse_from_rfc3339(&row.updated_at)
                    .expect("Invalid datetime")
                    .with_timezone(&chrono::Utc),
            )
        }))
    }

    async fn get_by_id(&self, id: UserId) -> Result<Option<User>, DbError> {
        let id_str = id.to_string();
        let result = sqlx::query!(
            r#"
            SELECT id, name, email, password, seikin_similarity, created_at, updated_at
            FROM users
            WHERE id = ?
            "#,
            id_str
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.map(|row| {
            let id = UserId::from_str(&row.id).expect("Invalid UUID");
            User::from_db(
                id,
                row.name,
                row.email,
                row.password,
                row.seikin_similarity,
                chrono::DateTime::parse_from_rfc3339(&row.created_at)
                    .expect("Invalid datetime")
                    .with_timezone(&chrono::Utc),
                chrono::DateTime::parse_from_rfc3339(&row.updated_at)
                    .expect("Invalid datetime")
                    .with_timezone(&chrono::Utc),
            )
        }))
    }
}
