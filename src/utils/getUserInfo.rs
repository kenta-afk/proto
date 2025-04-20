use reqwest::Client;
use crate::models::auth::User;

// This function retrieves the authenticated user's information from Backlog using the provided access token.
pub async fn get_authenticated_user(
    client: &Client,
    access_token: &str,
    space: &str,
) -> Result<User, reqwest::Error> {
    let url = format!("https://{space}.backlog.jp/api/v2/users/myself");
    let resp = client
        .get(&url)
        .bearer_auth(access_token)
        .send()
        .await?;

    resp.json::<User>().await
}