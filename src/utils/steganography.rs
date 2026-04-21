use axum::Json;
use crate::types::{Input, Output};

pub async fn encrypt(Json(payload): Json<Input>) -> Json<Output> {
    Json(Output {
        result: "this route will take an image and some plaintext and hide the plaintext in the image".to_string(),
    })
}

pub async fn decrypt(Json(payload): Json<Input>) -> Json<Output> {
    Json(Output {
        result: "this route will take an image and decrpt the hidden ciphertext".to_string(),
    })
}

