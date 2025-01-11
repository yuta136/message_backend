use crate::codes::error::ApiError;
use config;
use dotenvy::dotenv;
use serde::Deserialize;
use std::env;
use std::sync::OnceLock;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub app_environment: String,
    pub server_address: String,
    pub server_port: u16,
    pub db_url: String,
}

impl Config {
    fn from_env() -> Result<Self, ApiError> {
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new())?;
        cfg.try_into().map_err(|err| ApiError::from(err))
    }
}

pub static CONFIG: OnceLock<Config> = OnceLock::new();
pub fn set_config() -> Config {
    dotenv().ok();
    let environment = match env::var("APP_ENVIRONMENT") {
        Ok(val) => val,
        Err(_) => "development".to_string(),
    };
    if let Err(err) = dotenvy::from_filename(".env.".to_string() + &environment) {
        log::error!("Error: {}", err);
    }
    match Config::from_env() {
        Ok(config) => config,
        Err(err) => {
            log::error!("Config Error: {}", err);
            std::process::exit(1);
        }
    }
}
