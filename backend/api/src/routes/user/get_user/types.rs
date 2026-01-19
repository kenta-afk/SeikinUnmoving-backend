use serde::Serialize;

// GetUser用のレスポンス型
#[derive(Serialize)]
pub struct GetUserResponse {
    pub user_id: String,
    pub email: String,
    pub name: String,
    pub seikin_similarity: f64,
}
