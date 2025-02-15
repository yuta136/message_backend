use crate::codes::error::MessagesRepositoryError;
use crate::domain::model::message::Message;
use crate::domain::repository::messages_repository::MessagesRepisotoryTrait;
use crate::infrastructure::db::connection::DBConnection;
use crate::infrastructure::db::entity::messages;
use chrono::Utc;
use log::info;
use sea_orm::metric::Info;
use sea_orm::ActiveValue::NotSet;
use sea_orm::EntityTrait;
use sea_orm::Set;
use std::sync::Arc;

#[derive(Clone)]
pub struct MessageRepository {
    pub db_connection: Arc<DBConnection>,
}

impl MessagesRepisotoryTrait for MessageRepository {
    async fn save(&self, message: Message) -> Result<(), MessagesRepositoryError> {
        let conn = &self.db_connection.pool;

        let message_model = messages::ActiveModel {
            id: NotSet,
            message: Set(message.message),
            created_at: Set(Utc::now()),
        };

        messages::Entity::insert(message_model).exec(conn).await?;

        Ok(())
    }
}
