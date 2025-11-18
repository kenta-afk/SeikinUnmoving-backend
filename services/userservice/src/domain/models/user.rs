use crate::domain::{
    models::id::UserId,
    services::{secret_service::SecretService, uuid_service::UuidService},
};
use chrono::{DateTime, Utc};

pub struct User {
    pub id: UserId,
    pub name: String,
    pub email: String,
    pub password: String,
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
        let user_id = UserId::new(uuid_service);
        let hashed_password = secret_service.hash_password(&password);

        User {
            id: user_id,
            name,
            email,
            password: hashed_password,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }
}
