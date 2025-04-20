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
use base64::Engine;

use crate::models::auth::{TokenResponse, AuthConfig, UserIcon, UserWithIcon};
use crate::utils::getUserInfo::get_authenticated_user;
use crate::utils::getUserIcon::get_user_icon;

pub fn auth_routes(config: AuthConfig) -> Router {
    let config = Arc::new(config);
    Router::new()
        .route("/login", 
            get({
                let config = config.clone();
                move || login(config)
            }),
        )
        .route("/callback", 
            get({
                let config = config.clone();
                move |query| callback(query, config)
            }),
        )
}

/// `/login` ハンドラー: 認可コードを取得するためのリダイレクトを行う。
async fn login(config: Arc<AuthConfig>) -> Redirect {
    let encoded_redirect_url = urlencoding::encode(&config.redirect_uri);
    let url = format!(
        "https://{}.backlog.jp/OAuth2AccessRequest.action?response_type=code&client_id={}&redirect_uri={}&state=xyz",
        config.backlog_space, config.client_id, encoded_redirect_url,
    );
    Redirect::to(&url)
}

/// `/callback` ハンドラー: 認可コード受け取り→アクセストークン・ユーザー情報とアイコンを取得し返す。
async fn callback(
    Query(params): Query<HashMap<String, String>>,
    config: Arc<AuthConfig>,
) -> Result<String, (StatusCode, String)> {
    let code = params.get("code").cloned().ok_or_else(|| {
        (
            StatusCode::BAD_REQUEST,
            "認可コード(code)がクエリに含まれていません".into(),
        )
    })?;
    
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
    let text = resp
        .text()
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    
    if !status.is_success() {
        return Err((
            StatusCode::BAD_REQUEST,
            format!("トークン取得に失敗しました。ステータス: {}, レスポンス: {}", status, text),
        ));
    }
    
    let token: TokenResponse = serde_json::from_str(&text).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("トークンのパースに失敗: {}。受信データ: {}", e, text),
        )
    })?;
    
    // ユーザー情報の取得
    let user_info = get_authenticated_user_info(&client, &token.access_token, &config.backlog_space)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    // ユーザーアイコンの取得（失敗してもアイコンなしとして処理する）
    let user_icon = get_user_icon(&client, &token.access_token, &config.backlog_space, user_info.id)
        .await
        .ok();
    
    // アイコンデータをBase64にエンコード（取得できた場合）
    let icon_base64 = user_icon.map(|icon| {
        base64::engine::general_purpose::STANDARD.encode(icon.data)
    });
    
    // ユーザー情報とアイコンをまとめる
    let combined = UserWithIcon {
        user: user_info,
        icon_base64,
    };
    
    // JSONとして返す（必要に応じてContent-Typeを設定してください）
    serde_json::to_string_pretty(&combined)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}