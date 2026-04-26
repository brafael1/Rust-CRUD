use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub jwt: JwtConfig,
    pub argon2: Argon2Config,
    pub rate_limit: RateLimitConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub name: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout: u64,
    pub ssl_mode: String,
}

impl DatabaseConfig {
    pub fn connection_string(&self) -> String {
        let escaped_password = self
            .password
            .replace("%", "%25")
            .replace(":", "%3A")
            .replace("/", "%2F");
        format!(
            "postgres://{}:{}@{}:{}/{}?sslmode={}",
            self.username, escaped_password, self.host, self.port, self.name, self.ssl_mode
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisConfig {
    pub host: String,
    pub port: u16,
    pub username: Option<String>,
    pub password: Option<String>,
    pub db: i32,
    pub cache_ttl: u64,
}

impl RedisConfig {
    pub fn connection_string(&self) -> String {
        match (&self.username, &self.password) {
            (Some(user), Some(pass)) => {
                format!("redis://{}:{}@{}:{}", user, pass, self.host, self.port)
            }
            _ => format!("redis://{}:{}", self.host, self.port),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtConfig {
    pub secret: String,
    pub expiration: u64,
    pub issuer: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Argon2Config {
    pub memory: u32,
    pub iterations: u32,
    pub parallelism: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    pub requests_per_minute: u32,
    pub burst_size: u32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                host: "0.0.0.0".to_string(),
                port: 8080,
                workers: num_cpus(),
            },
            database: DatabaseConfig {
                host: "postgres".to_string(),
                port: 5432,
                username: "postgres".to_string(),
                password: "postgres".to_string(),
                name: "rust_crud".to_string(),
                max_connections: 5,
                min_connections: 0,
                connect_timeout: 60,
                ssl_mode: "disable".to_string(),
            },
            redis: RedisConfig {
                host: "redis".to_string(),
                port: 6379,
                username: None,
                password: None,
                db: 0,
                cache_ttl: 300,
            },
            jwt: JwtConfig {
                secret: "your-secret-key-change-in-production".to_string(),
                expiration: 3600,
                issuer: "rust-crud-api".to_string(),
            },
            argon2: Argon2Config {
                memory: 65536,
                iterations: 3,
                parallelism: 4,
            },
            rate_limit: RateLimitConfig {
                requests_per_minute: 100,
                burst_size: 10,
            },
        }
    }
}

fn num_cpus() -> usize {
    std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4)
}

pub fn load() -> Result<Settings, config::ConfigError> {
    let config_file = std::env::var("CONFIG_FILE").ok();

    let mut builder = config::Config::builder();

    if let Some(file) = config_file {
        builder = builder.add_source(config::File::with_name(&file));
    }

    builder
        .add_source(config::Environment::with_prefix("APP"))
        .build()?
        .try_deserialize()
}
