use uuid::Uuid;

use crate::{
    application::ports::uuid_service::UuidService,
    domain::models::id::{ClientId, UserId},
};

pub struct Client {
    pub id: ClientId,
    pub user_id: UserId,
    pub jti: Uuid,
    pub exp: i64,
    pub created_at: i64,
}

impl Client {
    pub fn new(user_id: UserId, jti: Uuid, exp: i64, uuid_service: &impl UuidService) -> Self {
        Self {
            id: ClientId::new(uuid_service),
            user_id,
            jti,
            exp,
            created_at: chrono::Utc::now().timestamp(),
        }
    }
    pub fn update(
        &mut self,
        user_id: UserId,
        jti: Uuid,
        exp: i64,
        uuid_service: &impl UuidService,
    ) -> Self {
        self.jti = jti;
        self.exp = exp;

        Client::new(user_id, jti, exp, uuid_service)
    }
}
