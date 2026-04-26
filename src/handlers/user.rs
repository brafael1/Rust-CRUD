use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, HeaderValue, StatusCode},
    Json,
    response::IntoResponse,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::config::AppState;
use crate::db::UserRepository;
use crate::errors::ApiError;
use crate::models::auth::TokenClaims;
use crate::models::response::{ApiResponse, PaginatedUsersResponse};
use crate::models::user::{CreateUserRequest, UpdateUserRequest, UserResponse};
use crate::services::user::UserService;

fn require_auth(headers: &HeaderMap) -> Result<TokenClaims, ApiError> {
    let auth = headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| ApiError::Unauthorized)?;

    if !auth.starts_with("Bearer ") {
        return Err(ApiError::Unauthorized);
    }

    let token = &auth[7..];
    let settings = crate::config::Settings::default();

    use jsonwebtoken::{decode, DecodingKey, Validation};

    let claims = decode::<TokenClaims>(
        token,
        &DecodingKey::from_secret(settings.jwt.secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| ApiError::Unauthorized)?
    .claims;

    Ok(claims)
}

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
    headers: HeaderMap,
    Json(payload): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, ApiError> {
    require_auth(&headers)?;

    let repository = UserRepository::new(&state.pool);
    let user_service = UserService::new(&repository, state.redis.clone(), state.settings.clone());
    
    let response = user_service.create(payload).await?;
    
    Ok((StatusCode::CREATED, Json(ApiResponse::new(response))))
}

pub async fn get_user(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    require_auth(&headers)?;

    let repository = UserRepository::new(&state.pool);
    let user_service = UserService::new(&repository, state.redis.clone(), state.settings.clone());
    
    let response = user_service.get_by_id(id).await?;
    
    Ok(Json(ApiResponse::new(response)))
}

pub async fn update_user(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<impl IntoResponse, ApiError> {
    require_auth(&headers)?;

    let repository = UserRepository::new(&state.pool);
    let user_service = UserService::new(&repository, state.redis.clone(), state.settings.clone());
    
    let response = user_service.update(id, payload.email, payload.username, payload.password).await?;
    
    Ok(Json(ApiResponse::new(response)))
}

pub async fn delete_user(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    require_auth(&headers)?;

    let repository = UserRepository::new(&state.pool);
    let user_service = UserService::new(&repository, state.redis.clone(), state.settings.clone());
    
    user_service.delete(id).await?;
    
    Ok(StatusCode::NO_CONTENT)
}

pub async fn list_users(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Query(query): Query<ListQuery>,
) -> Result<impl IntoResponse, ApiError> {
    require_auth(&headers)?;

    let repository = UserRepository::new(&state.pool);
    let user_service = UserService::new(&repository, state.redis.clone(), state.settings.clone());
    
    let response = user_service.list(query.cursor, query.limit.unwrap_or(20)).await?;
    
    Ok(Json(ApiResponse::new(response)))
}

pub async fn register_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let repository = UserRepository::new(&state.pool);
    let user_service = UserService::new(&repository, state.redis.clone(), state.settings.clone());
    
    let response = user_service.create(payload).await?;
    
    Ok((StatusCode::CREATED, Json(ApiResponse::new(response))))
}