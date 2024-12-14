use super::controllers;
use actix_web::{middleware::Logger, App, HttpServer};

#[actix_web::main]
pub async fn run() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(controllers::message)
    })
    .bind(format!("{}:{}", "127.0.0.1", "5765"))?
    .run()
    .await
}
