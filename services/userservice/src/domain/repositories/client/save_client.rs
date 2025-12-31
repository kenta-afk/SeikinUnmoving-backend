use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::{UserId, domain::models::id::ClientId};

pub struct SaveClient {
    pub id: ClientId,
    pub user_id: UserId,
    pub jti: Uuid,
    pub exp: i64,
    pub created_at: DateTime<Utc>,
}
