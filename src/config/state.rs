use std::{process::exit, time::Duration};

use envmode::EnvMode;
use redis::{Client as RedisClient, RedisError, aio::MultiplexedConnection};
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};

use super::ENV;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub rd: RedisClient,
}

impl AppState {
    pub async fn get_redis_conn<E>(&self) -> Result<MultiplexedConnection, E>
    where
        RedisError: Into<E>,
    {
        self.rd
            .get_multiplexed_async_connection()
            .await
            .map_err(Into::into)
    }
}

impl AppState {
    pub async fn new() -> Self {
        let mut opt = ConnectOptions::new(&*ENV.database_url);
        opt.max_lifetime(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(5))
            .sqlx_logging(true)
            .sqlx_logging_level(if EnvMode::is_prd(&ENV.env) {
                log::LevelFilter::Error
            } else {
                log::LevelFilter::Info
            })
            .set_schema_search_path(&*ENV.database_schema);

        let db: DatabaseConnection = Database::connect(opt).await.unwrap_or_else(|e| {
            log::error!("Failed to connect to database: {}", e);
            exit(1);
        });
        let rd = RedisClient::open(&*ENV.redis_url).unwrap_or_else(|e| {
            log::error!("Failed to connect to redis: {}", e);
            exit(1);
        });

        Self { db, rd }
    }
}

impl AppState {
    pub async fn shutdown(self) -> Result<(), DbErr> {
        self.db.close().await
    }
}
