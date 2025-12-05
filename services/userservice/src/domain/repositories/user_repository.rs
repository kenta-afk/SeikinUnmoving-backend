use crate::domain::models::{error::DbError, id::UserId, user::User};

#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait UserRepository: Send + Sync + Clone {
    async fn create(&self, user: User) -> Result<(), DbError>;
    async fn get_by_email(&self, email: &str) -> Result<Option<User>, DbError>;
    async fn get_by_id(&self, id: UserId) -> Result<Option<User>, DbError>;
}

#[cfg(test)]
impl Clone for MockUserRepository {
    fn clone(&self) -> Self {
        Self::default()
    }
}
