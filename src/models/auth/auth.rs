use serde::{Deserialize, Serialize};

// TokenResponse Structure
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u32,
    pub refresh_token: String,
}

// auth_config Structure
#[derive(Clone)]
pub struct AuthConfig {
    pub backlog_space: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
}

// User Structure
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: u64,
    pub user_id: String,        
    pub name: String,
    pub role_type: u32,         
    pub lang: Option<String>,
    pub nulab_account: Option<NulabAccount>,  
    pub mail_address: Option<String>,       
    pub last_login_time: Option<String>,   
}

// NulabAccount Structure
#[derive(Debug, Deserialize)]
pub struct NulabAccount {
    nulabId: String,
    name: String,
    uniqueId: String,
}