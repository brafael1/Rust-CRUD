use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
    #[serde(rename = "token_type")]
    pub token_type: String,
    #[serde(rename = "expires_in")]
    pub expires_in: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub user_id: String,
    pub iat: i64,
    pub exp: i64,
    pub iss: String,
}

impl TokenClaims {
    pub fn new(user_id: uuid::Uuid, email: String, issuer: &str, expiration: u64) -> Self {
        let now = chrono::Utc::now();
        let exp = now.timestamp() + expiration as i64;
        Self {
            sub: email,
            user_id: user_id.to_string(),
            iat: now.timestamp(),
            exp,
            iss: issuer.to_string(),
        }
    }
}
