use crate::domain::models::id::UserId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtClaims {
    pub sub: UserId,
}

impl JwtClaims {
    pub fn new(user_id: UserId, _expires_in_seconds: i64) -> Self {
        Self { sub: user_id }
    }
}
