use uuid::Uuid;

use crate::{
    application::ports::uuid_service::UuidService,
    domain::{
        models::id::{ClientId, UserId},
        repositories::client::{create_client::CreateClient, save_client::SaveClient},
    },
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Client {
    pub id: ClientId,
    pub user_id: UserId,
    pub jti: Uuid,
    pub exp: i64,
    pub created_at: DateTime<Utc>,
}

impl Client {
    pub fn new(user_id: UserId, jti: Uuid, exp: i64, uuid_service: &impl UuidService) -> Self {
        Self {
            id: ClientId::new(uuid_service),
            user_id,
            jti,
            exp,
            created_at: chrono::Utc::now(),
        }
    }
    pub fn update(&mut self, user_id: UserId, jti: Uuid, exp: i64) -> Self {
        Self {
            id: self.id,
            user_id,
            jti,
            exp,
            created_at: self.created_at,
        }
    }
    pub fn into_create(self) -> CreateClient {
        CreateClient {
            id: self.id,
            user_id: self.user_id,
            jti: self.jti,
            exp: self.exp,
            created_at: self.created_at,
        }
    }
    pub fn into_save(self) -> SaveClient {
        SaveClient {
            id: self.id,
            user_id: self.user_id,
            jti: self.jti,
            exp: self.exp,
            created_at: self.created_at,
        }
    }
    
    pub fn from_db(
        id: ClientId,
        user_id: UserId,
        jti: Uuid,
        exp: i64,
        created_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            user_id,
            jti,
            exp,
            created_at,
        }
    }
}
