use chrono::{DateTime, Utc};

use crate::domain::models::id::UserId;

pub struct DbUser {
    pub id: UserId,
    pub name: String,
    pub email: String,
    pub password: String,
    pub seikin_similarity: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
