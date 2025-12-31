use crate::domain::{
    models::{client::Client, error::DbError, id::UserId},
    repositories::client::{create_client::CreateClient, save_client::SaveClient},
};

#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait ClientRepository: Send + Sync + Clone + 'static {
    async fn create(&self, client: CreateClient) -> Result<(), DbError>;
    async fn get_by_user_id(&self, user_id: UserId) -> Result<Option<Client>, DbError>;
    async fn save(&self, client: SaveClient) -> Result<(), DbError>;
    async fn delete_by_user_id(&self, user_id: UserId) -> Result<(), DbError>;
}

#[cfg(test)]
impl Clone for MockClientRepository {
    fn clone(&self) -> Self {
        Self::default()
    }
}
