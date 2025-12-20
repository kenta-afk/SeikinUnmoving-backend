use uuid::Uuid;

use crate::{
    application::ports::uuid_service::UuidService,
    domain::{
        models::id::{ClientId, UserId},
        repositories::client::{create_client::CreateClient, save_client::SaveClient},
    },
};
use chrono::{DateTime, Utc};

pub struct Client {
    pub id: ClientId,
    pub user_id: UserId,
    pub jti: Uuid,
    pub exp: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl Client {
    pub fn new(
        user_id: UserId,
        jti: Uuid,
        exp: DateTime<Utc>,
        uuid_service: &impl UuidService,
    ) -> Self {
        Self {
            id: ClientId::new(uuid_service),
            user_id,
            jti,
            exp,
            created_at: chrono::Utc::now(),
        }
    }
    pub fn update(
        &mut self,
        user_id: UserId,
        jti: Uuid,
        exp: DateTime<Utc>,
        uuid_service: &impl UuidService,
    ) -> Self {
        self.jti = jti;
        self.exp = exp;

        Client::new(user_id, jti, exp, uuid_service)
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
}
