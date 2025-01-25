use crate::infrastructure::db::repositories::messages_repository::MessageRepository;

pub struct AppState {
    pub message_repository: MessageRepository,
}
