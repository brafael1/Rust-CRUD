use sqlx::{
    postgres::{PgPool, PgPoolOptions},
};
use std::time::Duration;
use tracing::{info, warn, error};

use crate::config::Settings;
use crate::errors::ApiError;

pub async fn create_pool(config: &Settings) -> Result<PgPool, ApiError> {
    let conn_str = config.database.connection_string();
    
    info!("[DBG] Connecting to {}@{}:{}/{}", 
        config.database.username,
        config.database.host, 
        config.database.port,
        config.database.name);
    
    info!("[DBG] Connection string: postgres://{}:***@{}/{}?sslmode={}",
        config.database.username,
        config.database.host,
        config.database.name,
        config.database.ssl_mode);
    
    info!("[DBG] max_connections={}, connect_timeout={}",
        config.database.max_connections,
        config.database.connect_timeout);

    let result = PgPoolOptions::new()
        .max_connections(config.database.max_connections)
        .acquire_timeout(Duration::from_secs(config.database.connect_timeout))
        .idle_timeout(Duration::from_secs(300))
        .max_lifetime(Duration::from_secs(1800))
        .connect(&conn_str)
        .await;
    
    match result {
        Ok(pool) => {
            info!("[DBG] Pool created successfully!");
            Ok(pool)
        }
        Err(e) => {
            error!("[DBG] Failed to create pool: {}", e);
            Err(ApiError::ServiceUnavailable(format!("Database error: {}", e)))
        }
    }
}

pub async fn check_connection(pool: &PgPool) -> Result<bool, sqlx::Error> {
    sqlx::query("SELECT 1")
        .execute(pool)
        .await
        .map(|_| true)
}