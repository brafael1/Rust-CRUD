use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use std::sync::Arc;

use crate::config::Settings;
use crate::db::UserRepository;
use crate::errors::ApiError;
use crate::models::auth::{LoginRequest, LoginResponse, TokenClaims};
use crate::models::user::{User, UserResponse};

pub struct AuthService<'a> {
    repository: &'a UserRepository<'a>,
    settings: Arc<Settings>,
}

impl<'a> AuthService<'a> {
    pub fn new(repository: &'a UserRepository<'a>, settings: Arc<Settings>) -> Self {
        Self { repository, settings }
    }

    pub async fn login(&self, request: LoginRequest) -> Result<LoginResponse, ApiError> {
        let user = self.repository.find_by_email(&request.email).await?;

        let parsed_hash = PasswordHash::new(&user.password_hash)
            .map_err(|_| ApiError::Internal("Invalid password hash format".to_string()))?;

        let argon2 = Argon2::default();
        argon2.verify_password(request.password.as_bytes(), &parsed_hash)
            .map_err(|_| ApiError::Unauthorized)?;

        let claims = TokenClaims::new(
            user.id,
            user.email.clone(),
            &self.settings.jwt.issuer,
            self.settings.jwt.expiration,
        );

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.settings.jwt.secret.as_bytes()),
        )
        .map_err(|e| ApiError::Internal(e.to_string()))?;

        Ok(LoginResponse {
            token,
            token_type: "Bearer".to_string(),
            expires_in: self.settings.jwt.expiration,
        })
    }

    pub fn verify_token(&self, token: &str) -> Result<TokenClaims, ApiError> {
        let decoded = decode::<TokenClaims>(
            token,
            &DecodingKey::from_secret(self.settings.jwt.secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| ApiError::Unauthorized)?;

        Ok(decoded.claims)
    }

    pub async fn hash_password(&self, password: &str) -> Result<String, ApiError> {
        let salt = SaltString::generate(&mut rand::thread_rng());
        let argon2 = Argon2::new(
            argon2::Algorithm::Argon2id,
            argon2::Version::V0x13,
            argon2::Params::new(
                self.settings.argon2.memory,
                self.settings.argon2.iterations,
                self.settings.argon2.parallelism,
                Some(128),
            ).map_err(|e| ApiError::Internal(e.to_string()))?,
        );

        let password_hash = argon2.hash_password(password.as_bytes(), &salt)
            .map_err(|e| ApiError::Internal(e.to_string()))?
            .to_string();

        Ok(password_hash)
    }
}