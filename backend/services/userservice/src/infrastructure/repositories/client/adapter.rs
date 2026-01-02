use crate::{
    domain::models::client::Client, infrastructure::repositories::client::db_client::DbClient,
};

impl From<DbClient> for Client {
    fn from(db_client: DbClient) -> Self {
        Client {
            id: db_client.id,
            user_id: db_client.user_id,
            jti: db_client.jti,
            exp: db_client.exp,
            created_at: db_client.created_at,
        }
    }
}
