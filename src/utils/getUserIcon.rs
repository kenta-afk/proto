use reqwest::Client;
use crate::models::auth::UserIcon;

/// Backlog APIからユーザーアイコンをダウンロードする関数
pub async fn get_user_icon(
    client: &Client,
    access_token: &str,
    space: &str,
    user_id: u64,
) -> Result<UserIcon, reqwest::Error> {
    // ユーザーIDをURL内に挿入
    let url = format!("https://{space}.backlog.jp/api/v2/users/{user_id}/icon", space = space, user_id = user_id);
    let resp = client
        .get(&url)
        .bearer_auth(access_token)
        .send()
        .await?;
    
    // レスポンスヘッダーからContent-TypeとContent-Dispositionを取得
    let headers = resp.headers();
    let content_type = headers
        .get("Content-Type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream")
        .to_string();
    let content_disposition = headers
        .get("Content-Disposition")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("attachment; filename=\"icon\"")
        .to_string();
    
    // レスポンスからバイト列を取得
    let data = resp.bytes().await?;
    
    Ok(UserIcon {
        content_type,
        content_disposition,
        data,
    })
}