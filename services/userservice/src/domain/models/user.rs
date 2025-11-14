use crate::domain::models::id::UserId;
use chrono::{DateTime, Utc};

#[allow(dead_code)]
pub struct User {
    pub id: UserId,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[allow(dead_code)]
impl User {
    fn new(
        id: UserId,
        name: String,
        email: String,
        password: String,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> User {
        User {
            id,
            name,
            email,
            password,
            created_at,
            updated_at,
        }
    }
}
