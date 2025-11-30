use crate::domain::models::id::{ClientId, UserId};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[allow(dead_code)]
pub struct DbClient {
    pub id: ClientId,
    pub user_id: UserId,
    pub jti: Uuid,
    pub exp: i64,
    pub created_at: DateTime<Utc>,
}
