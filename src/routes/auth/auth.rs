use axum::{
    Router,
    routing::get,
};

pub fn auth_routes() -> Router {
    Router::new()
        .route("/login", get(|| async { "Login" }))
}
