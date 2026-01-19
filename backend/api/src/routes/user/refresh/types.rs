use serde::{Deserialize, Serialize};

// Refresh用のリクエスト型
#[derive(Deserialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

// Refresh用のレスポンス型
#[derive(Serialize)]
pub struct RefreshResponse {
    pub user_id: String,
    pub email: String,
    pub name: String,
    pub seikin_similarity: f64,
    pub jwt: String,
    pub refresh_token: String,
}
