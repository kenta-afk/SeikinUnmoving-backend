use crate::{
    application::ports::secret_service::SecretService,
    domain::models::{jwt::JwtClaims, refresh_token::RefreshClaims},
};
use bcrypt::{hash, verify, DEFAULT_COST};
use jwt_simple::prelude::*;
use getrandom::getrandom;

#[derive(Clone)]
pub struct SecretServiceImpl {
    key: HS256Key,
}

impl SecretServiceImpl {
    #[allow(dead_code)]
    pub fn new(secret: &str) -> Self {
        Self {
            key: HS256Key::from_bytes(secret.as_bytes()),
        }
    }
}

impl SecretService for SecretServiceImpl {
    fn create_secret(&self) -> String {
        let mut bytes = [0u8; 32];
        getrandom(&mut bytes).expect("failed to generate random bytes");
        hex::encode(bytes)
    }
    fn hash_password(&self, password: &str) -> String {
        hash(password, DEFAULT_COST).expect("failed to hash password")
    }
    fn verify_password(&self, hashed: &str, password: &str) -> bool {
        verify(password, hashed).unwrap_or(false)
    }
    fn create_jwt(&self, claims: &JwtClaims) -> Result<String, String> {
        let jwt_claims = Claims::with_custom_claims(
            claims.clone(),
            Duration::from_secs((claims.exp - claims.iat) as u64),
        );
        self.key.authenticate(jwt_claims).map_err(|e| e.to_string())
    }
    fn decode_jwt(&self, token: &str) -> Result<JwtClaims, String> {
        let claims = self.key
            .verify_token::<JwtClaims>(token, None)
            .map_err(|e| e.to_string())?;
        Ok(claims.custom)
    }
    fn create_refresh_token(&self, claims: &RefreshClaims) -> Result<String, String> {
        let jwt_claims = Claims::with_custom_claims(
            claims.clone(),
            Duration::from_secs((claims.exp - claims.iat) as u64),
        );
        self.key.authenticate(jwt_claims).map_err(|e| e.to_string())
    }
    fn decode_refresh_token(&self, refresh_token: &str) -> Result<RefreshClaims, String> {
        let claims = self.key
            .verify_token::<RefreshClaims>(refresh_token, None)
            .map_err(|e| e.to_string())?;
        Ok(claims.custom)
    }
}