use crate::config::{set_config, CONFIG};

use super::controllers;
use actix_web::{middleware::Logger, App, HttpServer};

#[actix_web::main]
pub async fn run() -> std::io::Result<()> {
    let config = &CONFIG;
    let address = &config.get_or_init(set_config).server_address;
    let port = &config.get_or_init(set_config).server_port;
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
