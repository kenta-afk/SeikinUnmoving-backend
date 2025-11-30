use crate::domain::services::uuid_service::UuidService;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Copy, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct UserId(Uuid);

impl UserId {
    pub fn new(uuid_service: &impl UuidService) -> Self {
        Self(uuid_service.new_v7())
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct ClientId(Uuid);

impl ClientId {
    pub fn new(uuid_service: &impl UuidService) -> Self {
        Self(uuid_service.new_v7())
    }
}
