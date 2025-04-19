use crate::models::auth::AuthConfig;
use std::env;

pub fn load_auth_config() -> AuthConfig {
    AuthConfig {
        backlog_space: env::var("BACKLOG_SPACE").expect("BACKLOG_SPACE must be set"),
        client_id: env::var("CLIENT_ID").expect("CLIENT_ID must be set"),
        client_secret: env::var("CLIENT_SECRET").expect("CLIENT_SECRET must be set"),
        redirect_uri: env::var("REDIRECT_URI").expect("REDIRECT_URI must be set"),
    }
}