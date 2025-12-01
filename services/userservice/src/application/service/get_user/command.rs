use crate::domain::models::id::UserId;

pub struct GetUserCommand {
    pub user_id: UserId,
}
