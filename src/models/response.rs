use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedUsersResponse {
    pub data: Vec<super::user::UserResponse>,
    #[serde(rename = "next_cursor")]
    pub next_cursor: Option<String>,
    #[serde(rename = "has_more")]
    pub has_more: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub data: T,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}

impl<T> ApiResponse<T> {
    pub fn new(data: T) -> Self {
        Self { data }
    }
}

impl ErrorResponse {
    pub fn new(error: String, message: String) -> Self {
        Self { error, message }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    #[serde(rename = "database")]
    pub database: String,
    pub redis: String,
}
