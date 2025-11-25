use crate::{domain::models::user::User, infrastructure::repositories::dto::user::DbUser};

impl From<DbUser> for User {
    fn from(db_user: DbUser) -> Self {
        User {
            id: db_user.id,
            name: db_user.name,
            email: db_user.email,
            password: db_user.password,
            seikin_similarity: db_user.seikin_similarity,
            created_at: db_user.created_at,
            updated_at: db_user.updated_at,
        }
    }
}
