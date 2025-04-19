use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {

    let app = Router::new().route("/", get(|| async {"Hello, demo!"}));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let addr = listener.local_addr().unwrap();
    println!("Listening on http://{}", addr);
    axum::serve(listener, app).await.unwrap();
    
}