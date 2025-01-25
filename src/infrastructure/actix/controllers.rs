use crate::{
    app::usecase::receive_message::{ReceiveMessageCommand, ReceiveMessageUseCase},
    infrastructure::{
        actix::requests::{Message2Request, MessageRequest},
        app_state::AppState,
    },
};
use actix_web::{post, web, HttpResponse};
use serde_json::json;

#[post("/api/message")]
async fn message(
    data: web::Data<AppState>,
    request: web::Json<MessageRequest>,
) -> Result<HttpResponse, actix_web::error::Error> {
    let message1 = request.clone().message1;

    let message_command = ReceiveMessageCommand { message: message1 };

    let usecase = ReceiveMessageUseCase::new(data.message_repository.clone());

    usecase.save(message_command).await?;

    Ok(HttpResponse::Ok().into())
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
