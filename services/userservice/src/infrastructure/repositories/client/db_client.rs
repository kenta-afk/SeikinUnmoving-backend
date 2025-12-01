use crate::domain::models::id::{ClientId, UserId};
use uuid::Uuid;

pub struct DbClient {
    pub id: ClientId,
    pub user_id: UserId,
    pub jti: Uuid,
    pub exp: i64,
    pub created_at: i64,
}
