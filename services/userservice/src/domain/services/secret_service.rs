use crate::{domain::models::id::UserId, infrastructure::services::secret_service::JwtClaims};

#[cfg_attr(test, mockall::automock)]
#[allow(dead_code)]
pub trait SecretService: Send + Sync {
    fn create_secret(&self) -> String;
    fn hash_password(&self, password: &str) -> String;
    fn verify_password(&self, hashed: &str, password: &str) -> bool;
    fn create_jwt(&self, user_id: UserId) -> Result<String, jsonwebtoken::errors::Error>;
    fn decode_jwt(&self, token: &str) -> Result<JwtClaims, jsonwebtoken::errors::Error>;
}
