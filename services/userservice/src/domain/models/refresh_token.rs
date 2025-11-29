use crate::domain::{models::id::UserId, services::uuid_service::UuidService};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshClaims {
    pub sub: UserId,
    pub exp: usize,
    pub iat: usize,
    pub jti: Uuid,
}

impl RefreshClaims {
    pub fn new(user_id: UserId, uuid_service: &impl UuidService, expires_in_days: i64) -> Self {
        let now = chrono::Utc::now();
        Self {
            sub: user_id,
            exp: (now + chrono::Duration::days(expires_in_days)).timestamp() as usize,
            iat: now.timestamp() as usize,
            jti: uuid_service.new_v7(),
        }
    }
}
