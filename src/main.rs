use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};

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

#[tokio::main]
async fn main() {
    let app = Router::new().route("/process", post(process));
    let listener = tokio::net::TcpListener::bind("localhost:5555").await.unwrap();
    println!("running on localhost:5555");
    axum::serve(listener, app).await.unwrap();
}
