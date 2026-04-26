use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::cache::RedisCache;
use crate::config::{AppState, Settings};
use crate::db::UserRepository;
use crate::errors::ApiError;
use crate::models::auth::LoginRequest;
use crate::models::response::ApiResponse;
use crate::models::user::UserResponse;
use crate::services::auth::AuthService;

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<ApiResponse<crate::models::auth::LoginResponse>>, ApiError> {
    let repository = UserRepository::new(&state.pool);
    let auth_service = AuthService::new(&repository, state.settings.clone());
    
    let response = auth_service.login(request).await?;
    
    Ok(Json(ApiResponse::new(response)))
}

pub async fn logout(
    State(state): State<Arc<AppState>>,
) -> StatusCode {
    StatusCode::OK
}