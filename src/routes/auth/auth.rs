use axum::{
    Router,
    routing::get,
    extract::Query,
    response::Redirect,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use reqwest::Client;
use std::collections::HashMap;

// Backlog OAuth2 設定 (環境変数管理を推奨)
const BACKLOG_SPACE: &str   = "nulab";
const CLIENT_ID: &str       = "KlYJOvnQsP79UHlOrc9Bh64daHgtGv1j";
const CLIENT_SECRET: &str   = "YiQDdqfvhSauzpfkU9vAmlJhlx8FCI3aD3rN9xhw8jwDDTnkTAkJkn7wR9gbQZ3w";
const REDIRECT_URI: &str    = "http://localhost:3000/callback";

// トークンレスポンス
#[derive(Debug, Serialize, Deserialize)]
struct TokenResponse {
    access_token: String,
    token_type:   String,
    expires_in:   u32,
    refresh_token:String,
}


/// 認可フロー用ルートをまとめて返す
pub fn auth_routes() -> Router {
    Router::new()
        .route("/login",    get(login_handler))
        .route("/callback", get(callback_handler))
}

/// `/login` ハンドラー: Backlog 認可エンドポイントへリダイレクト
async fn login_handler() -> Redirect {
    // 確実に URL エンコードする
    let encoded_redirect = urlencoding::encode(REDIRECT_URI);

    // {} を使ってプレースホルダに変数を渡す
    // .jp と統一
    let url = format!(
        "https://{}.backlog.jp/OAuth2AccessRequest.action\
         ?response_type=code\
         &client_id={}\
         &redirect_uri={}\
         &state=xyz",
        BACKLOG_SPACE,
        CLIENT_ID,
        encoded_redirect,
    );

    Redirect::to(&url)
}


/// `/callback` ハンドラー: 認可コード受け取り→トークン交換
async fn callback_handler(
    Query(params): Query<HashMap<String, String>>,
) -> Result<String, (StatusCode, String)> {
    // クエリパラメータから code を取り出し
    let code = if let Some(code) = params.get("code") {
        code.clone()
    } else {
        // code がない場合はユーザーが直接アクセスしたか異常
        return Err((StatusCode::BAD_REQUEST, "認可コード(code)がクエリに含まれていません".into()));
    };

    // トークンエンドポイントへ POST
    let client = Client::new();
    let resp = client.post(format!(
            "https://{space}.backlog.jp/api/v2/oauth2/token",
            space = BACKLOG_SPACE,
        ))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&[
            ("grant_type",    "authorization_code"),
            ("code",          &code),
            ("redirect_uri",  REDIRECT_URI),
            ("client_id",     CLIENT_ID),
            ("client_secret", CLIENT_SECRET),
        ])
        .send()
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    // エラーデバッグのために生のレスポンスを取得
    let status = resp.status();
    let text = resp.text().await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    // レスポンスのステータスコードを確認
    if !status.is_success() {
        return Err((
            StatusCode::BAD_REQUEST, 
            format!("トークン取得に失敗しました。ステータス: {}, レスポンス: {}", status, text)
        ));
    }

    // JSONへのパースを試みる
    match serde_json::from_str::<TokenResponse>(&text) {
        Ok(token) => Ok(format!("トークン取得成功: {:#?}", token)),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("トークンのパースに失敗: {}。受信データ: {}", e, text)
        )),
    }
}