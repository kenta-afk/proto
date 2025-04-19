mod routes;
mod models;
mod env;

use axum::{
    routing::get,
    Router,
};

use dotenvy::dotenv;
use routes::auth::auth_routes;

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    let config = env::load_auth_config();

    let app = Router::new()
        .route("/", get(|| async { "Hello, demo!" }))
        .merge(auth_routes(config));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let addr = listener.local_addr().unwrap();

    println!("Listening on http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}