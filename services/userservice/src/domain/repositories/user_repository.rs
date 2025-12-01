use crate::domain::models::{error::DbError, id::UserId, user::User};

#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
#[allow(dead_code)]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: User) -> Result<(), DbError>;
    async fn get_by_email(&self, email: &str) -> Result<Option<User>, DbError>;
    async fn get_by_id(&self, id: UserId) -> Result<Option<User>, DbError>;
}
