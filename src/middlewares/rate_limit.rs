use std::sync::Arc;

use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

use crate::config::Settings;

pub struct RateLimiter {
    requests: Arc<std::sync::RwLock<HashMap<String, (Vec<Instant>, u32)>>>,
    config: Arc<Settings>,
}

impl RateLimiter {
    pub fn new(config: Arc<Settings>) -> Self {
        Self {
            requests: Arc::new(std::sync::RwLock::new(HashMap::new())),
            config,
        }
    }

    pub fn check(&self, client_ip: &str) -> Result<(), ()> {
        let mut requests = self.requests.write().unwrap();

        let (timestamps, _) = requests
            .entry(client_ip.to_string())
            .or_insert((Vec::new(), self.config.rate_limit.burst_size));

        let now = Instant::now();
        let window = Duration::from_secs(60);

        timestamps.retain(|t| now.duration_since(*t) < window);

        if timestamps.len() >= self.config.rate_limit.requests_per_minute as usize {
            return Err(());
        }

        timestamps.push(now);
        Ok(())
    }
}
