use crate::{
    domain::{
        models::{
            client::Client,
            error::DbError,
            id::UserId,
        },
        repositories::client::{
            client_repository::ClientRepository, create_client::CreateClient,
            save_client::SaveClient,
        },
    },
};

#[derive(Clone)]
pub struct ClientRepositoryImpl {
    // WASM環境では、実際のDB接続の代わりにモックや外部APIを使用
}

#[allow(dead_code)]
impl ClientRepositoryImpl {
    pub fn new() -> Self {
        Self {}
    }
}

#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
impl ClientRepository for ClientRepositoryImpl {
    async fn create(&self, _client: CreateClient) -> Result<(), DbError> {
        // WASM環境では、Cloudflare Workers D1 APIを使用する実装に置き換える
        Err(DbError::Generic("Not implemented for WASM".to_string()))
    }
    async fn get_by_user_id(&self, _user_id: UserId) -> Result<Option<Client>, DbError> {
        // WASM環境では、Cloudflare Workers D1 APIを使用する実装に置き換える
        Err(DbError::Generic("Not implemented for WASM".to_string()))
    }
    async fn save(&self, _client: SaveClient) -> Result<(), DbError> {
        // WASM環境では、Cloudflare Workers D1 APIを使用する実装に置き換える
        Err(DbError::Generic("Not implemented for WASM".to_string()))
    }
    async fn delete_by_user_id(&self, _user_id: UserId) -> Result<(), DbError> {
        // WASM環境では、Cloudflare Workers D1 APIを使用する実装に置き換える
        Err(DbError::Generic("Not implemented for WASM".to_string()))
    }
}
