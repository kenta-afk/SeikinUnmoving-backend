use crate::domain::models::id::UserId;

#[allow(dead_code)]
pub struct GetUserDto {
    pub user_id: UserId,
    pub email: String,
    pub name: String,
    pub seikin_similarity: f64,
}
