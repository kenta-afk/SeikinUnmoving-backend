use crate::domain::models::{jwt::JwtClaims, refresh_token::RefreshClaims};

#[cfg_attr(test, mockall::automock)]
pub trait SecretService: Send + Sync + Clone + 'static {
    fn create_secret(&self) -> String;
    fn hash_password(&self, password: &str) -> String;
    fn verify_password(&self, hashed: &str, password: &str) -> bool;
    fn create_jwt(&self, claims: &JwtClaims) -> Result<String, jsonwebtoken::errors::Error>;
    fn decode_jwt(&self, token: &str) -> Result<JwtClaims, jsonwebtoken::errors::Error>;
    fn create_refresh_token(
        &self,
        claims: &RefreshClaims,
    ) -> Result<String, jsonwebtoken::errors::Error>;
    fn decode_refresh_token(
        &self,
        refresh_token: &str,
    ) -> Result<RefreshClaims, jsonwebtoken::errors::Error>;
}

#[cfg(test)]
impl Clone for MockSecretService {
    fn clone(&self) -> Self {
        Self::default()
    }
}
