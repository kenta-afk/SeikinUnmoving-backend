use crate::domain::models::{client::Client, error::DbError, id::UserId};

#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait ClientRepository: Send + Sync + Clone + 'static {
    async fn create(&self, client: Client) -> Result<(), DbError>;
    async fn get_by_user_id(&self, user_id: UserId) -> Result<Option<Client>, DbError>;
    async fn save(&self, client: Client) -> Result<(), DbError>;
}

#[cfg(test)]
impl Clone for MockClientRepository {
    fn clone(&self) -> Self {
        Self::default()
    }
}
