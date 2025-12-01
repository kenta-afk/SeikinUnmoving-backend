use crate::{
    application::ports::{secret_service::SecretService, uuid_service::UuidService},
    domain::models::id::UserId,
};
use chrono::{DateTime, Utc};

#[derive(sqlx::Type)]
pub struct User {
    pub id: UserId,
    pub name: String,
    pub email: String,
    pub password: String,
    pub seikin_similarity: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(
        name: String,
        email: String,
        password: String,
        uuid_service: &impl UuidService,
        secret_service: &impl SecretService,
    ) -> User {
        User {
            id: UserId::new(uuid_service),
            name,
            email,
            password: secret_service.hash_password(&password),
            seikin_similarity: 0.0,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }
}
