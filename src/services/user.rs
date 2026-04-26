use std::sync::Arc;

use uuid::Uuid;

use crate::cache::RedisCache;
use crate::config::Settings;
use crate::db::UserRepository;
use crate::errors::ApiError;
use crate::models::response::PaginatedUsersResponse;
use crate::models::user::{CreateUserRequest, User, UserResponse};

pub struct UserService<'a> {
    repository: &'a UserRepository<'a>,
    cache: Option<Arc<RedisCache>>,
    settings: Arc<Settings>,
}

impl<'a> UserService<'a> {
    pub fn new(
        repository: &'a UserRepository<'a>,
        cache: Option<Arc<RedisCache>>,
        settings: Arc<Settings>,
    ) -> Self {
        Self {
            repository,
            cache,
            settings,
        }
    }

    pub async fn create(&self, request: CreateUserRequest) -> Result<UserResponse, ApiError> {
        self.validate_create_request(&request)?;

        let password_hash = self.hash_password(&request.password).await?;

        let user = User::new(request.email, request.username, password_hash);
        let result = self.repository.create(&user).await?;

        self.invalidate_cache().await;

        Ok(result)
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<UserResponse, ApiError> {
        if let Some(ref cache) = self.cache {
            let cache_key = format!("user:{}", id);
            if let Ok(Some(cached)) = cache.get::<UserResponse>(&cache_key).await {
                tracing::info!("Cache hit for user {}", id);
                return Ok(cached);
            }
        }

        let result = self.repository.find_by_id(id).await?;

        if let Some(ref cache) = self.cache {
            let cache_key = format!("user:{}", id);
            if let Err(e) = cache.set(&cache_key, &result).await {
                tracing::warn!("Failed to cache user: {}", e);
            }
        }

        Ok(result)
    }

    pub async fn update(
        &self,
        id: Uuid,
        email: Option<String>,
        username: Option<String>,
        password: Option<String>,
    ) -> Result<UserResponse, ApiError> {
        let password_hash = if let Some(ref pwd) = password {
            Some(self.hash_password(pwd).await?)
        } else {
            None
        };

        let result = self.repository.update(id, email, username, password_hash).await?;

        self.invalidate_user_cache(id).await;
        self.invalidate_list_cache().await;

        Ok(result)
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), ApiError> {
        self.repository.delete(id).await?;
        self.invalidate_user_cache(id).await;
        self.invalidate_list_cache().await;
        Ok(())
    }

    pub async fn list(
        &self,
        cursor: Option<String>,
        limit: i64,
    ) -> Result<PaginatedUsersResponse, ApiError> {
        let cache_key = format!("users:list:{}:{}", cursor.as_deref().unwrap_or("none"), limit);
        let cursor_is_none = cursor.is_none();

        if let Some(ref cache) = self.cache {
            if cursor_is_none {
                match cache.get::<PaginatedUsersResponse>(&cache_key).await {
                    Ok(Some(cached)) => {
                        tracing::info!("Cache hit for user list");
                        return Ok(cached);
                    }
                    _ => {}
                }
            }
        }

        let (users, next_cursor) = self.repository.list_cursor(cursor, limit).await?;
        let has_more = next_cursor.is_some();

        let result = PaginatedUsersResponse {
            data: users,
            next_cursor,
            has_more,
        };

        if let Some(ref cache) = self.cache {
            if cursor_is_none {
                if let Err(e) = cache.set(&cache_key, &result).await {
                    tracing::warn!("Failed to cache user list: {}", e);
                }
            }
        }

        Ok(result)
    }

    fn validate_create_request(&self, request: &CreateUserRequest) -> Result<(), ApiError> {
        if request.email.is_empty() || !request.email.contains('@') {
            return Err(ApiError::BadRequest("Invalid email format".to_string()));
        }

        if request.username.len() < 3 || request.username.len() > 50 {
            return Err(ApiError::BadRequest("Username must be between 3 and 50 characters".to_string()));
        }

        if request.password.len() < 8 {
            return Err(ApiError::BadRequest("Password must be at least 8 characters".to_string()));
        }

        Ok(())
    }

    async fn hash_password(&self, password: &str) -> Result<String, ApiError> {
        use argon2::{
            password_hash::{PasswordHasher, SaltString},
            Argon2,
        };

        let salt = SaltString::generate(&mut rand::thread_rng());
        let argon2 = Argon2::default();

        Ok(argon2.hash_password(password.as_bytes(), &salt)
            .map_err(|e| ApiError::Internal(e.to_string()))?
            .to_string())
    }

    async fn invalidate_user_cache(&self, id: Uuid) {
        if let Some(ref cache) = self.cache {
            let cache_key = format!("user:{}", id);
            if let Err(e) = cache.delete(&cache_key).await {
                tracing::warn!("Failed to invalidate user cache: {}", e);
            }
        }
    }

    async fn invalidate_list_cache(&self) {
        if let Some(ref cache) = self.cache {
            if let Err(e) = cache.delete_pattern("users:list:*").await {
                tracing::warn!("Failed to invalidate list cache: {}", e);
            }
        }
    }

    async fn invalidate_cache(&self) {
        self.invalidate_list_cache().await;
    }
}