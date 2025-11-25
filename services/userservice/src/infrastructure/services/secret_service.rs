use crate::domain::{models::id::UserId, services::secret_service::SecretService};
use argon2::{
    Argon2, PasswordHasher, PasswordVerifier,
    password_hash::{PasswordHash, SaltString},
};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, encode};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    sub: UserId,
    exp: usize,
    iat: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct RefreshClaims {
    sub: UserId,
    exp: usize,
    iat: usize,
    jti: Uuid,
}

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
    fn create_jwt(&self, user_id: UserId) -> Result<String, jsonwebtoken::errors::Error> {
        let now = chrono::Utc::now();
        let claims = JwtClaims {
            sub: user_id,
            exp: (now + chrono::Duration::hours(24)).timestamp() as usize,
            iat: now.timestamp() as usize,
        };

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
}
