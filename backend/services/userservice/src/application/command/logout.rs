use crate::domain::models::id::UserId;

pub struct LogoutCommand {
    pub user_id: UserId,
}
