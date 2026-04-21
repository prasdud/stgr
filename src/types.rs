use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Input {
    pub data: String,
}

#[derive(Serialize)]
pub struct Output {
    pub result: String,
}
