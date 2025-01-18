use crate::config::{set_config, CONFIG};
use sea_orm::ConnectOptions;
use sea_orm::Database;
use sea_orm::DatabaseConnection;
use std::time::Duration;

#[derive(Clone)]
pub struct DBConnection {
    pub pool: DatabaseConnection,
}
pub async fn db_connect() -> DBConnection {
    let db_url = &CONFIG.get_or_init(set_config).db_url;
    let mut opt = ConnectOptions::new(db_url);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8));

    let pool: DatabaseConnection = Database::connect(opt).await.unwrap();

    DBConnection { pool }
}
