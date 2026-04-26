use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};

use serde::Deserialize;
use uuid::Uuid;

use crate::config::AppState;
use crate::db::UserRepository;
use crate::errors::ApiError;
use crate::models::response::{ApiResponse, PaginatedUsersResponse};
use crate::models::user::{CreateUserRequest, UpdateUserRequest, UserResponse};
use crate::services::user::UserService;

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub cursor: Option<String>,
    pub limit: Option<i64>,
}

impl Default for ListQuery {
    fn default() -> Self {
        Self {
            cursor: None,
            limit: Some(20),
        }
    }
}

pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<ApiResponse<UserResponse>>), ApiError> {
    let repository = UserRepository::new(&state.pool);
    let user_service = UserService::new(&repository, state.redis.clone(), state.settings.clone());
    
    let response = user_service.create(request).await?;
    
    Ok((StatusCode::CREATED, Json(ApiResponse::new(response))))
}

pub async fn get_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<UserResponse>>, ApiError> {
    let repository = UserRepository::new(&state.pool);
    let user_service = UserService::new(&repository, state.redis.clone(), state.settings.clone());
    
    let response = user_service.get_by_id(id).await?;
    
    Ok(Json(ApiResponse::new(response)))
}

pub async fn update_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateUserRequest>,
) -> Result<Json<ApiResponse<UserResponse>>, ApiError> {
    let repository = UserRepository::new(&state.pool);
    let user_service = UserService::new(&repository, state.redis.clone(), state.settings.clone());
    
    let response = user_service.update(id, request.email, request.username, request.password).await?;
    
    Ok(Json(ApiResponse::new(response)))
}

pub async fn delete_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    let repository = UserRepository::new(&state.pool);
    let user_service = UserService::new(&repository, state.redis.clone(), state.settings.clone());
    
    user_service.delete(id).await?;
    
    Ok(StatusCode::NO_CONTENT)
}

pub async fn list_users(
    State(state): State<Arc<AppState>>,
    Query(query): Query<ListQuery>,
) -> Result<Json<ApiResponse<PaginatedUsersResponse>>, ApiError> {
    let repository = UserRepository::new(&state.pool);
    let user_service = UserService::new(&repository, state.redis.clone(), state.settings.clone());
    
    let response = user_service.list(query.cursor, query.limit.unwrap_or(20)).await?;
    
    Ok(Json(ApiResponse::new(response)))
}