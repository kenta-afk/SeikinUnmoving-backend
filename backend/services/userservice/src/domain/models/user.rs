use crate::{
    GetUserDto,
    application::ports::{secret_service::SecretService, uuid_service::UuidService},
    domain::{models::id::UserId, repositories::user::create_user::CreateUser},
    infrastructure::repositories::user::db_user::DbUser,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    id: UserId,
    name: String,
    email: String,
    password: String,
    seikin_similarity: f64,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
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
    pub fn id(&self) -> UserId {
        self.id
    }
    pub fn password(&self) -> &str {
        &self.password
    }
    pub fn into_get(self) -> GetUserDto {
        GetUserDto {
            user_id: self.id,
            email: self.email,
            name: self.name,
            seikin_similarity: self.seikin_similarity,
        }
    }
    pub fn into_create(self) -> CreateUser {
        CreateUser {
            id: self.id,
            name: self.name,
            email: self.email,
            password: self.password,
            seikin_similarity: self.seikin_similarity,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
    pub fn from_db(db_user: DbUser) -> Self {
        User {
            id: db_user.id,
            name: db_user.name,
            email: db_user.email,
            password: db_user.password,
            seikin_similarity: db_user.seikin_similarity,
            created_at: db_user.created_at,
            updated_at: db_user.updated_at,
        }
    }
}
