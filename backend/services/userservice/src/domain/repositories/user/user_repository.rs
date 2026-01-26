use crate::domain::{
    models::{error::DbError, id::UserId, user::User},
    repositories::user::create_user::CreateUser,
};

#[cfg_attr(test, mockall::automock)]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
pub trait UserRepository: Clone + Send + Sync + 'static {
    async fn create(&self, user: CreateUser) -> Result<(), DbError>;
    async fn get_by_email(&self, email: &str) -> Result<Option<User>, DbError>;
    async fn get_by_id(&self, id: UserId) -> Result<Option<User>, DbError>;
    async fn update_seikin_similarity(&self, id: UserId, similarity: f64) -> Result<(), DbError>;
}

#[cfg(test)]
impl Clone for MockUserRepository {
    fn clone(&self) -> Self {
        Self::default()
    }
}
