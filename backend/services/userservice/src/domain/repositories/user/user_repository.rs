use crate::domain::{
    models::{error::DbError, id::UserId, user::User},
    repositories::user::create_user::CreateUser,
};

#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait UserRepository: Send + Sync + Clone + 'static {
    async fn create(&self, user: CreateUser) -> Result<(), DbError>;
    async fn get_by_email(&self, email: &str) -> Result<Option<User>, DbError>;
    async fn get_by_id(&self, id: UserId) -> Result<Option<User>, DbError>;
}

#[cfg(test)]
impl Clone for MockUserRepository {
    fn clone(&self) -> Self {
        Self::default()
    }
}
