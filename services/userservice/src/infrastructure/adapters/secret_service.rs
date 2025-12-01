use crate::{
    application::ports::secret_service::SecretService,
    domain::models::{jwt::JwtClaims, refresh_token::RefreshClaims},
};
use argon2::{
    Argon2, PasswordHasher, PasswordVerifier,
    password_hash::{PasswordHash, SaltString},
};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, encode};
use rand::RngCore;

#[allow(dead_code)]
pub struct SecretServiceImpl {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl SecretServiceImpl {
    #[allow(dead_code)]
    pub fn new(secret: &str) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
        }
    }
}

impl SecretService for SecretServiceImpl {
    fn create_secret(&self) -> String {
        let mut rng = rand::rng();
        let mut bytes = [0u8; 32];
        rng.fill_bytes(&mut bytes);
        hex::encode(bytes)
    }
    fn hash_password(&self, password: &str) -> String {
        let salt = SaltString::generate();
        let argon2 = Argon2::default();

        argon2
            .hash_password(password.as_bytes(), &salt)
            .expect("failed to hash password")
            .to_string()
    }
    fn verify_password(&self, hashed: &str, password: &str) -> bool {
        let parsed_hash = PasswordHash::new(hashed).ok();

        match parsed_hash {
            Some(hash) => Argon2::default()
                .verify_password(password.as_bytes(), &hash)
                .is_ok(),
            None => false,
        }
    }
    fn create_jwt(&self, claims: &JwtClaims) -> Result<String, jsonwebtoken::errors::Error> {
        encode(&Header::default(), &claims, &self.encoding_key)
    }
    fn decode_jwt(&self, token: &str) -> Result<JwtClaims, jsonwebtoken::errors::Error> {
        let token_data = jsonwebtoken::decode::<JwtClaims>(
            token,
            &self.decoding_key,
            &jsonwebtoken::Validation::new(Algorithm::HS256),
        )?;
        Ok(token_data.claims)
    }
    fn create_refresh_token(
        &self,
        claims: &RefreshClaims,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        encode(&Header::default(), claims, &self.encoding_key)
    }
    fn decode_refresh_token(
        &self,
        refresh_token: &str,
    ) -> Result<RefreshClaims, jsonwebtoken::errors::Error> {
        let token_data = jsonwebtoken::decode::<RefreshClaims>(
            refresh_token,
            &self.decoding_key,
            &jsonwebtoken::Validation::new(Algorithm::HS256),
        )?;
        Ok(token_data.claims)
    }
}
