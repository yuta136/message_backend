use crate::infrastructure::actix::requests::{Message2Request, MessageRequest};
use actix_web::{post, web, HttpResponse};
use serde_json::json;

#[post("/api/message")]
async fn message(
    request: web::Json<MessageRequest>,
) -> Result<HttpResponse, actix_web::error::Error> {
    let message1 = request.clone().message1;
    let message2 = request.clone().message2;

    let mut message1_for_concat = message1.to_owned();
    let message2_for_concat = message2.to_owned();
    message1_for_concat.push_str(&message2_for_concat);

    Ok(HttpResponse::Ok().json(json!(
        {"message": message1_for_concat}
    )))
}

// 新しいエンドポイントをここに

#[post("/api/message_2")]
async fn message_2(
    request: web::Json<Message2Request>,
) -> Result<HttpResponse, actix_web::error::Error> {
    let user_id = request.clone().user_id;
    let message_a = request.clone().message;

    let response_message = format!("{}から{}が来ました", user_id, message_a);

    Ok(HttpResponse::Ok().json(json!(
        {"message_2":response_message}
    )))
}
