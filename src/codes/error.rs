use config::ConfigError;
use sea_orm::DbErr;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Config Error")]
    ConfigError(#[from] ConfigError),
}

#[derive(Debug, Serialize)]
pub struct ApiReponseBody {
    pub error_message: String,
}

#[derive(Debug, Error)]
pub enum MessagesRepositoryError {
    #[error("Database Error")]
    SeaOrmError(#[from] DbErr),
}

impl actix_web::error::ResponseError for MessagesRepositoryError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        actix_web::HttpResponse::InternalServerError().json(ApiReponseBody {
            error_message: self.to_string(),
        })
    }
}
