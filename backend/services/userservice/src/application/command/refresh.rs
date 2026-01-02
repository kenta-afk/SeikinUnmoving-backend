use crate::domain::models::id::UserId;
use uuid::Uuid;

pub struct RefreshCommand {
    pub user_id: UserId,
    pub jti: Uuid,
}
