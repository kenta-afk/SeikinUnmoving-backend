use crate::domain::{models::id::UserId, services::uuid_service::UuidService};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshClaims {
    pub sub: UserId,
    pub exp: i64,
    pub iat: i64,
    pub jti: Uuid,
}

impl RefreshClaims {
    pub fn new(user_id: UserId, uuid_service: &impl UuidService, expires_in_days: i64) -> Self {
        let now = chrono::Utc::now();
        Self {
            sub: user_id,
            exp: (now + chrono::Duration::days(expires_in_days)).timestamp(),
            iat: now.timestamp(),
            jti: uuid_service.new_v7(),
        }
    }
}
