use crate::config::{set_config, CONFIG};
use crate::infrastructure::db::connection::db_connect;
use std::sync::Arc;

use super::controllers;
use actix_web::{middleware::Logger, App, HttpServer};

#[actix_web::main]
pub async fn run() -> std::io::Result<()> {
    let config = &CONFIG;
    let address = &config.get_or_init(set_config).server_address;
    let port = &config.get_or_init(set_config).server_port;
    let db_connection = Arc::new(db_connect().await);
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(controllers::message)
            .service(controllers::message_2)
    })
    .bind(format!("{}:{}", address, port))?
    .run()
    .await
}
