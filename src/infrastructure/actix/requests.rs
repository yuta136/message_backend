use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct MessageRequest {
    pub message1: String,
    pub message2: String,

    pub user_id: String,
    pub message_a: String,
}
