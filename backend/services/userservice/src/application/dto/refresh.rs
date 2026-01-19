use crate::domain::models::id::UserId;

pub struct RefreshDto {
    pub user_id: UserId,
    pub email: String,
    pub name: String,
    pub seikin_similarity: f64,
    pub jwt: String,
    pub refresh_token: String,
}
