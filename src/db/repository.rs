use chrono::{DateTime, Utc};
use sqlx::{postgres::PgPool, types::Uuid, Row};

use crate::errors::ApiError;
use crate::models::user::{User, UserResponse};

pub struct UserRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> UserRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, user: &User) -> Result<UserResponse, ApiError> {
        let result = sqlx::query(
            "INSERT INTO users (id, email, username, password_hash, created_at, updated_at) 
             VALUES ($1, $2, $3, $4, $5, $6) 
             RETURNING id, email, username, created_at, updated_at"
        )
        .bind(user.id)
        .bind(&user.email)
        .bind(&user.username)
        .bind(&user.password_hash)
        .bind(user.created_at)
        .bind(user.updated_at)
        .fetch_one(self.pool)
        .await
        .map_err(|e| {
            if let Some(db_err) = e.as_database_error() {
                if db_err.constraint() == Some("users_email_key") {
                    return ApiError::Conflict("Email already exists".to_string());
                } else if db_err.constraint() == Some("users_username_key") {
                    return ApiError::Conflict("Username already exists".to_string());
                }
            }
            ApiError::Internal(e.to_string())
        })?;

        Ok(UserResponse {
            id: result.get("id"),
            email: result.get("email"),
            username: result.get("username"),
            created_at: result.get("created_at"),
            updated_at: result.get("updated_at"),
        })
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<UserResponse, ApiError> {
        let result = sqlx::query(
            "SELECT id, email, username, created_at, updated_at FROM users WHERE id = $1"
        )
        .bind(id)
        .fetch_one(self.pool)
        .await
        .map_err(|e| {
            if e.to_string().contains("row") && e.to_string().contains("not") {
                ApiError::NotFound("User not found".to_string())
            } else {
                ApiError::Internal(e.to_string())
            }
        })?;

        Ok(UserResponse {
            id: result.get("id"),
            email: result.get("email"),
            username: result.get("username"),
            created_at: result.get("created_at"),
            updated_at: result.get("updated_at"),
        })
    }

    pub async fn find_by_email(&self, email: &str) -> Result<User, ApiError> {
        let result = sqlx::query(
            "SELECT id, email, username, password_hash, created_at, updated_at FROM users WHERE email = $1"
        )
        .bind(email)
        .fetch_one(self.pool)
        .await
        .map_err(|e| {
            if e.to_string().contains("row") && e.to_string().contains("not") {
                ApiError::NotFound("User not found".to_string())
            } else {
                ApiError::Internal(e.to_string())
            }
        })?;

        Ok(User {
            id: result.get("id"),
            email: result.get("email"),
            username: result.get("username"),
            password_hash: result.get("password_hash"),
            created_at: result.get("created_at"),
            updated_at: result.get("updated_at"),
        })
    }

    pub async fn update(&self, id: Uuid, email: Option<String>, username: Option<String>, password_hash: Option<String>) -> Result<UserResponse, ApiError> {
        let existing = sqlx::query(
            "SELECT id, email, username, password_hash, created_at, updated_at FROM users WHERE id = $1"
        )
        .bind(id)
        .fetch_one(self.pool)
        .await
        .map_err(|_| ApiError::NotFound("User not found".to_string()))?;

        let new_email = email.unwrap_or_else(|| existing.get("email"));
        let new_username = username.unwrap_or_else(|| existing.get("username"));
        let new_password_hash = password_hash.unwrap_or_else(|| existing.get("password_hash"));
        let now = Utc::now();

        let result = sqlx::query(
            "UPDATE users SET email = $1, username = $2, password_hash = $3, updated_at = $4 
             WHERE id = $5 
             RETURNING id, email, username, created_at, updated_at"
        )
        .bind(&new_email)
        .bind(&new_username)
        .bind(&new_password_hash)
        .bind(now)
        .bind(id)
        .fetch_one(self.pool)
        .await
        .map_err(|e| {
            if let Some(db_err) = e.as_database_error() {
                if db_err.constraint() == Some("users_email_key") {
                    return ApiError::Conflict("Email already exists".to_string());
                } else if db_err.constraint() == Some("users_username_key") {
                    return ApiError::Conflict("Username already exists".to_string());
                }
            }
            ApiError::Internal(e.to_string())
        })?;

        Ok(UserResponse {
            id: result.get("id"),
            email: result.get("email"),
            username: result.get("username"),
            created_at: result.get("created_at"),
            updated_at: result.get("updated_at"),
        })
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), ApiError> {
        let result = sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id)
            .execute(self.pool)
            .await
            .map_err(|e| ApiError::Internal(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(ApiError::NotFound("User not found".to_string()));
        }

        Ok(())
    }

    pub async fn list_cursor(
        &self,
        cursor: Option<String>,
        limit: i64,
    ) -> Result<(Vec<UserResponse>, Option<String>), ApiError> {
        let cursor_id: Option<Uuid> = cursor
            .and_then(|c| Uuid::parse_str(&c).ok());

        let rows = if let Some(cursor_id) = cursor_id {
            let cursor_timestamp = sqlx::query(
                "SELECT created_at FROM users WHERE id = $1"
            )
            .bind(cursor_id)
            .fetch_one(self.pool)
            .await
            .map_err(|_| ApiError::NotFound("Cursor user not found".to_string()))?;

            let ts: DateTime<Utc> = cursor_timestamp.get("created_at");

            sqlx::query(
                "SELECT id, email, username, created_at, updated_at 
                 FROM users 
                 WHERE (created_at, id) < ($1, $2)
                 ORDER BY created_at DESC, id DESC 
                 LIMIT $3"
            )
            .bind(ts)
            .bind(cursor_id)
            .bind(limit + 1)
            .fetch_all(self.pool)
            .await
            .map_err(|e| ApiError::Internal(e.to_string()))?
        } else {
            sqlx::query(
                "SELECT id, email, username, created_at, updated_at 
                 FROM users 
                 ORDER BY created_at DESC, id DESC 
                 LIMIT $1"
            )
            .bind(limit + 1)
            .fetch_all(self.pool)
            .await
            .map_err(|e| ApiError::Internal(e.to_string()))?
        };

        let has_more = rows.len() > limit as usize;
        let rows = if has_more {
            &rows[..rows.len() - 1]
        } else {
            &rows
        };

        let users: Vec<UserResponse> = rows
            .iter()
            .map(|row| UserResponse {
                id: row.get("id"),
                email: row.get("email"),
                username: row.get("username"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
            .collect();

        let next_cursor = if has_more {
            users.last().map(|u| u.id.to_string())
        } else {
            None
        };

        Ok((users, next_cursor))
    }
}