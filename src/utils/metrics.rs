use std::sync::atomic::{AtomicUsize, Ordering};

use serde::Serialize;

pub static REQUEST_COUNT: AtomicUsize = AtomicUsize::new(0);

#[derive(Serialize)]
pub struct MetricsResponse {
    pub requests_total: usize,
}

pub fn increment_requests() {
    REQUEST_COUNT.fetch_add(1, Ordering::Relaxed);
}

pub fn get_requests_total() -> usize {
    REQUEST_COUNT.load(Ordering::Relaxed)
}
