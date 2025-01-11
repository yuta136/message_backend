use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct MessageRequest {
    pub message1: String,
    pub message2: String,
}

#[derive(Deserialize, Clone)]
pub struct Message2Request {
    pub user_id: String,
    pub message: String,
}
