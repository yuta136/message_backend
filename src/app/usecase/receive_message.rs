use crate::{
    codes::error::MessagesRepositoryError,
    domain::{
        model::message::{self, Message},
        repository::messages_repository::MessagesRepisotoryTrait,
    },
};

pub struct ReceiveMessageCommand {
    pub message: String,
}

pub struct ReceiveMessageUseCase<S> {
    message_repository: S,
}

impl<S> ReceiveMessageUseCase<S>
where
    S: MessagesRepisotoryTrait,
{
    pub fn new(message_repository: S) -> Self {
        Self { message_repository }
    }

    pub async fn save(
        &self,
        command: ReceiveMessageCommand,
    ) -> Result<(), MessagesRepositoryError> {
        let message = Message {
            message: command.message,
        };

        self.message_repository.save(message).await?;

        Ok(())
    }
}
