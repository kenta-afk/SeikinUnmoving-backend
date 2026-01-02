use axum::extract::FromRef;
use jsonwebtoken::{Algorithm, DecodingKey, Validation};

#[derive(Clone)]
pub struct JwtConfig {
    pub decoding_key: DecodingKey,
    pub validation: Validation,
}

impl JwtConfig {
    pub fn new(secret_key: &str) -> Self {
        Self {
            decoding_key: DecodingKey::from_secret(secret_key.as_bytes()),
            validation: Validation::new(Algorithm::HS256),
        }
    }
}

#[derive(Clone)]
pub struct AppState<US>
where
    US: Clone,
{
    pub user_service: US,
    pub jwt_config: JwtConfig,
}

#[derive(Clone)]
pub struct UserServiceState<T>(pub T);

impl<US> FromRef<AppState<US>> for UserServiceState<US>
where
    US: Clone,
{
    fn from_ref(state: &AppState<US>) -> Self {
        Self(state.user_service.clone())
    }
}

impl<US> FromRef<AppState<US>> for JwtConfig
where
    US: Clone,
{
    fn from_ref(state: &AppState<US>) -> Self {
        state.jwt_config.clone()
    }
}
