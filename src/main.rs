mod routes;

use axum::{
    routing::get,
    Router,
};

use routes::auth::auth_routes;


#[tokio::main]
async fn main() {

    let app = Router::new()
        .route("/", get(|| async {"Hello, demo!"}))
        .merge(auth_routes());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let addr = listener.local_addr().unwrap();
    
    println!("Listening on http://{}", addr);
    axum::serve(listener, app).await.unwrap();
    
}