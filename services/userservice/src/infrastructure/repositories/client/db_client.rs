use crate::domain::models::id::{ClientId, UserId};
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct DbClient {
    pub id: ClientId,
    pub user_id: UserId,
    pub jti: Uuid,
    pub exp: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}
