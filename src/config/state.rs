use std::sync::Arc;

use crate::cache::RedisCache;
use crate::config::Settings;

pub struct AppState {
    pub pool: sqlx::PgPool,
    pub settings: Arc<Settings>,
    pub redis: Option<Arc<RedisCache>>,
}

impl AppState {
    pub fn new(
        pool: sqlx::PgPool,
        settings: Arc<Settings>,
        redis: Option<Arc<RedisCache>>,
    ) -> Self {
        Self {
            pool,
            settings,
            redis,
        }
    }
}

impl Clone for AppState {
    fn clone(&self) -> Self {
        Self {
            pool: self.pool.clone(),
            settings: self.settings.clone(),
            redis: self.redis.clone(),
        }
    }
}
