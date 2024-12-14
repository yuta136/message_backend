use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct MessageRequest {
    pub message1: String,
    pub message2: String,
}