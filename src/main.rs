use axum::{routing::post, routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Deserialize)]
struct Input {
    data: String,
}

#[derive(Serialize)]
struct Output{
    result: String,
}

async fn process(Json(payload): Json<Input>) -> Json<Output> {
    Json(Output {
        result: format!("processed: {}", payload.data),
    })
}

async fn encrypt(Json(payload): Json<Input>) -> Json<Output> {
    Json(Output {
        result: "this route will take an image and some plaintext and hide the plaintext in the image".to_string(),
    })
}

async fn decrypt(Json(payload): Json<Input>) -> Json<Output> {
    Json(Output {
        result: "this route will take an image and decrpt the hidden ciphertext".to_string(),
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
        .route("/encrypt", post(encrypt))
        .route("/decrypt", post(decrypt));
        
    let listener = tokio::net::TcpListener::bind("localhost:5555").await.unwrap();
    println!("running on localhost:5555");
    axum::serve(listener, app).await.unwrap();
}
