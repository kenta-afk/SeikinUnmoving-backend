use crate::domain::models::{client::Client, error::DbError};

#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
#[allow(dead_code)]
pub trait ClientRepository: Send + Sync {
    async fn save(&self, client: Client) -> Result<(), DbError>;
}
