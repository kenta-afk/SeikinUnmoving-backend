use crate::{
    domain::{
        models::{error::DbError, id::UserId, user::User},
        repositories::user::{create_user::CreateUser, user_repository::UserRepository},
    },
};

#[derive(Clone)]
pub struct UserRepositoryImpl {
    // WASM環境では、実際のDB接続の代わりにモックや外部APIを使用
    // pool: SqlitePool を削除
}

#[allow(dead_code)]
impl UserRepositoryImpl {
    pub fn new() -> Self {
        Self {}
    }
}

#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
impl UserRepository for UserRepositoryImpl {
    async fn create(&self, _user: CreateUser) -> Result<(), DbError> {
        // WASM環境では、Cloudflare Workers D1 APIを使用する実装に置き換える
        Err(DbError::Generic("Not implemented for WASM".to_string()))
    }
    async fn get_by_email(&self, _email: &str) -> Result<Option<User>, DbError> {
        // WASM環境では、Cloudflare Workers D1 APIを使用する実装に置き換える
        Err(DbError::Generic("Not implemented for WASM".to_string()))
    }
    async fn get_by_id(&self, _id: UserId) -> Result<Option<User>, DbError> {
        // WASM環境では、Cloudflare Workers D1 APIを使用する実装に置き換える
        Err(DbError::Generic("Not implemented for WASM".to_string()))
    }
}
