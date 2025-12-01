use crate::domain::models::{error::DbError, user::User};

#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
#[allow(dead_code)]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: User) -> Result<(), DbError>;
    async fn get_by_email(&self, email: &str) -> Result<Option<User>, DbError>;
}
