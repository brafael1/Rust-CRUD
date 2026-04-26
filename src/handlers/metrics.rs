use axum::{response::Html, routing::get, Router};
use prometheus::{Encoder, TextEncoder};

pub fn routes() -> Router {
    Router::new().route("/metrics", get(metrics))
}

pub async fn metrics() -> Html<String> {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    
    let output = String::from_utf8(buffer).unwrap();
    Html(output)
}