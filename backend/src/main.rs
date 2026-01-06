use axum::{
    extract::State,
    routing::{get, post}, // Add 'post' here
    Json, Router,
};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;
use mongodb::{Client, Collection, bson::Document}; // Added Document
use std::sync::Arc;
use futures::stream::StreamExt; // This will work after you run 'cargo add'

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

    //1. Connect to MongoDB
    let uri = "mongodb://localhost:27017";
    let client = Client::with_uri_str(uri).await.expect("Failed to connect to MongoDB!");
    let db = client.database("perfume_db");
    let collection = db.collection::<Perfume>("perfumes");


    // We wrap the collection in an Arc so it can be shared safely between routes
    let shared_collection = Arc::new(collection);

    let cors = CorsLayer::permissive();

    // 2. Build the app and "hand over" the database to our routes
    let app = Router::new()
        .route("/", get(|| async { "Perfume Inventory API is Online!" }))
        .route("/api/perfumes", get(get_perfumes))
        .route("/api/perfumes", post(create_perfume))
        .layer(cors)
        .with_state(shared_collection); // This shares the DB with the functions below

    // 3. Define the address (localhost port 8000)
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("🚀 Server running at http://{}", addr);

    // 4. Start the server
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


async fn create_perfume(
    State(collection): State<Arc<Collection<Perfume>>>,
    Json(mut new_perfume): Json<Perfume>,
) -> Json<Perfume> {
    // 1. Insert the perfume into the MongoDB collection
    let result = collection
        .insert_one(&new_perfume)
        .await
        .expect("Failed to insert perfume");

    // 2. Get the ID that MongoDB generated and assign it to our object
    new_perfume.id = Some(result.inserted_id.as_object_id().unwrap());

    // 3. Return the saved perfume back to the user to confirm it worked
    Json(new_perfume)
}

async fn get_perfumes(
    State(collection): State<Arc<Collection<Perfume>>>,
) -> Json<Vec<Perfume>> {
    // This asks MongoDB: "Give me everything in the perfume drawer"
    let mut cursor = collection.find(Document::new()).await.expect("Failed to fetch perfumes");
    let mut perfumes = Vec::new();

    // We loop through the results and add them to our list
    while let Some(result) = cursor.next().await {
        if let Ok(perfume) = result {
            perfumes.push(perfume);
        }
    }

    Json(perfumes) // Now it returns what is ACTUALLY in the database!
}