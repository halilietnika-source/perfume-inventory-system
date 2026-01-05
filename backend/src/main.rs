use axum::{routing::get, Json, Router};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

// 2. DATA STRUCTURES: This is the "Blueprint" for a Perfume
#[derive(Debug, Serialize, Deserialize)]
pub struct Perfume {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub brand: String,
    pub price: f64,
    pub stock: i32,

}

// 3. THE MAIN ENGINE: This starts your server
#[tokio::main]
async fn main() {
    let cors = CorsLayer::permissive();

    // 2. Build our application with a "Health Check" route
    let app = Router::new()
        .route("/", get(|| async { "Perfume Inventory API is Online!" }))
        .route("/api/perfumes", get(get_perfumes))
        .layer(cors);

    // 3. Define the address (localhost port 8000)
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("🚀 Server running at http://{}", addr);

    // 4. Start the server
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// This function creates "mock" data to test our API
async fn get_perfumes() -> Json<Vec<Perfume>>{

let p1 = Perfume{
    id: None,
    name: "Bleu de Chanel".to_string(),
    brand: "Chanel".to_string(),
    price: 7.0,
    stock: 100,
};

let p2 = Perfume{
id: None,
name: "Sauvage".to_string(),
brand: "Dior".to_string(),
price: 10.0,
stock: 70,
};

Json(vec![p1,p2]) // Returns the list as JSON


}