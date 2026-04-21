use axum::{routing::post, routing::get, Json, Router};
use time::OffsetDateTime;

mod types;
mod utils;

use types::{Input, Output};

async fn process(Json(payload): Json<Input>) -> Json<Output> {
    Json(Output {
        result: format!("processed: {}", payload.data),
    })
}

async fn health() -> Json<Output> {
    Json(Output {
        result: OffsetDateTime::now_utc().to_string()
    })
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(health))
        .route("/process", post(process))
        .route("/encrypt", post(utils::steganography::encrypt))
        .route("/decrypt", post(utils::steganography::decrypt));
        
    let listener = tokio::net::TcpListener::bind("localhost:5555").await.unwrap();
    println!("running on localhost:5555");
    axum::serve(listener, app).await.unwrap();
}
