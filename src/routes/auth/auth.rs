use axum::{
    Router,
    routing::get,
    extract::Query,
    response::Redirect,
    http::StatusCode,
};
use reqwest::Client;
use std::collections::HashMap;
use std::sync::Arc;

use crate::models::auth::{TokenResponse, AuthConfig};

/// 認可フロー用ルートをまとめて返す
pub fn auth_routes(config: AuthConfig) -> Router {
    let config = Arc::new(config);
    Router::new()
        .route("/login", 
            get({
                let config = config.clone();
                move || login_handler(config)
            }),
        )
        .route("/callback", 
            get({
                let config = config.clone();
                move |query| callback_handler(query, config)
            }),
        )
}

// / `/login` ハンドラー: 認可コードを取得するためのリダイレクトを行う。
async fn login_handler(config: Arc<AuthConfig>) -> Redirect {
    let encoded_redirect_url = urlencoding::encode(&config.redirect_uri);

    let url = format!(
        "https://{}.backlog.jp/OAuth2AccessRequest.action?response_type=code&client_id={}&redirect_uri={}&state=xyz",
        config.backlog_space, config.client_id, encoded_redirect_url,
    );

    Redirect::to(&url)
}

/// `/callback` ハンドラー: 認可コード受け取り→アクセストークンを取得する。
async fn callback_handler(
    Query(params): Query<HashMap<String, String>>,
    config: Arc<AuthConfig>,
) -> Result<String, (StatusCode, String)> {
    let code = if let Some(code) = params.get("code") {
        code.clone()
    } else {
        return Err((StatusCode::BAD_REQUEST, "認可コード(code)がクエリに含まれていません".into()));
    };

    let client = Client::new();
    let resp = client
        .post(format!(
            "https://{space}.backlog.jp/api/v2/oauth2/token",
            space = config.backlog_space,
        ))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&[
            ("grant_type", "authorization_code"),
            ("code", &code),
            ("redirect_uri", &config.redirect_uri),
            ("client_id", &config.client_id),
            ("client_secret", &config.client_secret),
        ])
        .send()
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    let status = resp.status();
    let text = resp.text().await.map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    if !status.is_success() {
        return Err((
            StatusCode::BAD_REQUEST,
            format!("トークン取得に失敗しました。ステータス: {}, レスポンス: {}", status, text),
        ));
    }

    match serde_json::from_str::<TokenResponse>(&text) {
        Ok(token) => Ok(format!("トークン取得成功: {:#?}", token)),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("トークンのパースに失敗: {}。受信データ: {}", e, text),
        )),
    }
}