use actix_web::{get, HttpResponse};
use serde_json::json;

#[get("/api/hello")]
async fn hello() -> Result<HttpResponse, actix_web::error::Error> {
    Ok(HttpResponse::Ok().json(json!(
        {"message": "Hello World!"}
    )))
}
#[get("/api/rock")]
async fn rock() -> Result<HttpResponse, actix_web::error::Error> {
    Ok(HttpResponse::Ok().json(json!(
        {"message":"paper"}
    )))
}
#[get("/api/paper")]
async fn paper() -> Result<HttpResponse, actix_web::error::Error> {
    Ok(HttpResponse::Ok().json(json!(
        {"message":"scissors"}
    )))
}
#[get("/api/scissors")]
async fn scissors() -> Result<HttpResponse, actix_web::error::Error> {
    Ok(HttpResponse::Ok().json(json!(
        {"message":"rock"}
    )))
}
