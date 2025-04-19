use serde::{Deserialize, Serialize};

// トークンレスポンス
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u32,
    pub refresh_token: String,
}

// 環境変数をまとめた構造体
#[derive(Clone)]
pub struct AuthConfig {
    pub backlog_space: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
}