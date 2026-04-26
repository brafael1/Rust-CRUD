use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Invalid credentials")]
    Unauthorized,

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Internal server error: {0}")]
    Internal(String),

    #[error("Too many requests")]
    RateLimited,

    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error, message) = match self {
            ApiError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                "unauthorized".to_string(),
                "Invalid credentials".to_string(),
            ),
            ApiError::NotFound(resource) => {
                (StatusCode::NOT_FOUND, "not_found".to_string(), resource)
            }
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, "bad_request".to_string(), msg),
            ApiError::Conflict(msg) => (StatusCode::CONFLICT, "conflict".to_string(), msg),
            ApiError::Internal(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error".to_string(),
                msg,
            ),
            ApiError::RateLimited => (
                StatusCode::TOO_MANY_REQUESTS,
                "rate_limited".to_string(),
                "Too many requests".to_string(),
            ),
            ApiError::ServiceUnavailable(msg) => (
                StatusCode::SERVICE_UNAVAILABLE,
                "service_unavailable".to_string(),
                msg,
            ),
        };

        let body = json!({
            "error": error,
            "message": message,
        });

        (status, Json(body)).into_response()
    }
}
