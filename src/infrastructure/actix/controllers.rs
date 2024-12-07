use actix_web::{get, HttpResponse};
use serde_json::json;

#[get("/api/hello")]
async fn hello() -> Result<HttpResponse, actix_web::error::Error> {
    Ok(HttpResponse::Ok().json(json!(
        {"message": "Hello World!"}
    )))
}
