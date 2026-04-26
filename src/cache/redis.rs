use redis::{
    aio::ConnectionManager,
    AsyncCommands, Client, RedisResult,
};
use serde::{de::DeserializeOwned, Serialize};

use crate::config::Settings;

pub async fn create_redis_client(config: &Settings) -> RedisResult<ConnectionManager> {
    let client = Client::open(config.redis.connection_string())?;
    let connection_manager = ConnectionManager::new(client).await?;
    Ok(connection_manager)
}

pub struct RedisCache {
    manager: ConnectionManager,
    ttl: u64,
}

impl RedisCache {
    pub fn new(manager: ConnectionManager, ttl: u64) -> Self {
        Self { manager, ttl }
    }

    pub async fn get<T: DeserializeOwned>(&self, key: &str) -> RedisResult<Option<T>> {
        let mut conn = self.manager.clone();
        let value: Option<String> = conn.get(key).await?;
        
        if let Some(s) = value {
            match serde_json::from_str(&s) {
                Ok(deserialized) => Ok(Some(deserialized)),
                Err(_) => Ok(None),
            }
        } else {
            Ok(None)
        }
    }

    pub async fn set<T: Serialize>(&self, key: &str, value: &T) -> Result<(), String> {
        let mut conn = self.manager.clone();
        let serialized = serde_json::to_string(value).map_err(|e| e.to_string())?;
        
        redis::cmd("SETEX")
            .arg(key)
            .arg(self.ttl)
            .arg(serialized)
            .query_async::<_, ()>(&mut conn)
            .await
            .map_err(|e| e.to_string())?;
        
        Ok(())
    }

    pub async fn delete(&self, key: &str) -> RedisResult<()> {
        let mut conn = self.manager.clone();
        let _ = conn.del::<&str, i32>(key).await;
        Ok(())
    }

    pub async fn delete_pattern(&self, pattern: &str) -> RedisResult<()> {
        let mut conn = self.manager.clone();
        let keys: Vec<String> = conn.keys(pattern).await?;
        
        if !keys.is_empty() {
            let _ = conn.del::<Vec<String>, i32>(keys).await;
        }
        
        Ok(())
    }

    pub async fn health_check(&self) -> RedisResult<bool> {
        let mut conn = self.manager.clone();
        let result: String = redis::cmd("PING")
            .query_async::<_, String>(&mut conn)
            .await?;
        Ok(result == "PONG")
    }
}