use crate::codes::error::MessagesRepositoryError;
use crate::domain::model::message::Message;

pub trait MessagesRepisotoryTrait {
    async fn save(&self, message: Message) -> Result<(), MessagesRepositoryError>;
}
