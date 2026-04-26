mod cache;
mod config;
mod db;
mod errors;
mod handlers;
mod middlewares;
mod models;
mod services;
mod utils;

use std::sync::Arc;

use axum::{
    routing::{delete, get, post, put},
    Router,
};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

use cache::{create_redis_client, RedisCache};
use config::{AppState, Settings};
use db::{check_connection, create_pool};

async fn health(
    state: axum::extract::State<Arc<AppState>>,
) -> axum::Json<models::response::HealthResponse> {
    let db_status = match check_connection(&state.pool).await {
        Ok(true) => "healthy",
        _ => "unhealthy",
    }
    .to_string();

    let redis_status = if let Some(ref redis) = state.redis.as_ref() {
        match redis.health_check().await {
            Ok(true) => "healthy",
            _ => "unhealthy",
        }
    } else {
        "not_configured"
    }
    .to_string();

    axum::Json(models::response::HealthResponse {
        status: "ok".to_string(),
        database: db_status,
        redis: redis_status,
    })
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(fmt::layer().with_target(true))
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .init();

    tracing::info!("Starting Rust CRUD API");

    let settings = Arc::new(config::load().unwrap_or_else(|_| Settings::default()));

    tracing::info!("Creating PostgreSQL connection pool");
    let pool = create_pool(&settings).await?;
    tracing::info!("PostgreSQL connection pool created");

    let redis = match create_redis_client(&settings).await {
        Ok(manager) => {
            tracing::info!("Redis connection established");
            Some(Arc::new(RedisCache::new(manager, settings.redis.cache_ttl)))
        }
        Err(e) => {
            tracing::warn!("Failed to connect to Redis: {}. Continuing without cache.", e);
            None
        }
    };

    let app_state = Arc::new(AppState::new(pool, settings.clone(), redis));

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/health", get(health))
        .route("/auth/login", post(handlers::auth::login))
        .route("/users", post(handlers::user::create_user))
        .route("/users", get(handlers::user::list_users))
        .route("/users/:id", get(handlers::user::get_user))
        .route("/users/:id", put(handlers::user::update_user))
        .route("/users/:id", delete(handlers::user::delete_user))
        .route("/metrics", get(handlers::metrics::metrics))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);

    let addr = SocketAddr::from(([0, 0, 0, 0], settings.server.port));
    let listener = TcpListener::bind(addr).await?;
    tracing::info!("Listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}