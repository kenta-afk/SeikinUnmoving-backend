use crate::domain::models::id::UserId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    pub sub: UserId,
    pub exp: usize,
    pub iat: usize,
}

impl JwtClaims {
    pub fn new(user_id: UserId, expires_in_seconds: i64) -> Self {
        let now = chrono::Utc::now();
        Self {
            sub: user_id,
            exp: (now + chrono::Duration::seconds(expires_in_seconds)).timestamp() as usize,
            iat: now.timestamp() as usize,
        }
    }
}
