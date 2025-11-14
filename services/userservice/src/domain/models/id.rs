use crate::domain::services::uuid_service::UuidService;
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::Type)]
#[sqlx(transparent)]
pub struct UserId(Uuid);

#[allow(dead_code)]
impl UserId {
    pub fn new(uuid_service: &impl UuidService) -> Self {
        UserId(uuid_service.new_v7())
    }
}
