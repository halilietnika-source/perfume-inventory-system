use axum::{routing::get, Router};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    // 1. Setup CORS so your Vue app can connect later
    // Permissive CORS is great for development
    let cors = CorsLayer::permissive();

    // 2. Build our application with a "Health Check" route
    let app = Router::new()
        .route("/", get(|| async { "Perfume Inventory API is Online!" }))
        .layer(cors);

    // 3. Define the address (localhost port 8000)
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("🚀 Server running at http://{}", addr);

    // 4. Start the server
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}