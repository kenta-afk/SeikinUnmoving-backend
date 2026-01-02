use chrono::{DateTime, Utc};

use crate::UserId;

pub struct CreateUser {
    pub id: UserId,
    pub name: String,
    pub email: String,
    pub password: String,
    pub seikin_similarity: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
