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
pub struct AppState<US, GS>
where
    US: Clone,
    GS: Clone,
{
    pub user_service: US,
    pub game_service: GS,
    pub jwt_config: JwtConfig,
}

#[derive(Clone)]
pub struct UserServiceState<T>(pub T);

#[derive(Clone)]
pub struct GameServiceState<T>(pub T);

impl<US, GS> FromRef<AppState<US, GS>> for UserServiceState<US>
where
    US: Clone,
    GS: Clone,
{
    fn from_ref(state: &AppState<US, GS>) -> Self {
        Self(state.user_service.clone())
    }
}

impl<US, GS> FromRef<AppState<US, GS>> for GameServiceState<GS>
where
    US: Clone,
    GS: Clone,
{
    fn from_ref(state: &AppState<US, GS>) -> Self {
        Self(state.game_service.clone())
    }
}

impl<US, GS> FromRef<AppState<US, GS>> for JwtConfig
where
    US: Clone,
    GS: Clone,
{
    fn from_ref(state: &AppState<US, GS>) -> Self {
        state.jwt_config.clone()
    }
}
