use config::ConfigError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Config Error")]
    ConfigError(#[from] ConfigError),
}
