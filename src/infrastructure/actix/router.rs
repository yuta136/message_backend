use crate::config::{set_config, CONFIG};
use crate::infrastructure::app_state::AppState;
use crate::infrastructure::db::connection::db_connect;
use crate::infrastructure::db::repositories::messages_repository::MessageRepository;
use std::sync::Arc;

use super::controllers;
use crate::infrastructure::db::connection::DBConnection;
use actix_web::{middleware::Logger, web, App, HttpServer};

#[actix_web::main]
pub async fn run() -> std::io::Result<()> {
    let config = &CONFIG;
    let address = &config.get_or_init(set_config).server_address;
    let port = &config.get_or_init(set_config).server_port;
    let db_connection = Arc::new(db_connect().await);
    HttpServer::new(move || {
        let data = crate_data(db_connection.clone());
        App::new()
            .app_data(data)
            .wrap(Logger::default())
            .service(controllers::message)
            .service(controllers::message_2)
    })
    .bind(format!("{}:{}", address, port))?
    .run()
    .await
}

fn crate_data(db_connection: Arc<DBConnection>) -> web::Data<AppState> {
    web::Data::new(AppState {
        message_repository: MessageRepository {
            db_connection: db_connection.clone(),
        },
    })
}
