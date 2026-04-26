use axum::{
    body::Body,
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use std::sync::Arc;

use crate::config::Settings;
use crate::models::auth::TokenClaims;

pub async fn auth_middleware(mut request: Request, next: Next) -> Response {
    let auth_header = request
        .headers()
        .get("authorization")
        .and_then(|v| v.to_str().ok());

    let token = match auth_header {
        Some(header) if header.starts_with("Bearer ") => &header[7..],
        _ => {
            return Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body(Body::empty())
                .unwrap();
        }
    };

    let settings = request
        .extensions()
        .get::<Arc<Settings>>()
        .cloned();

    let settings = match settings {
        Some(s) => s,
        None => {
            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::empty())
                .unwrap();
        }
    };

    use jsonwebtoken::{decode, DecodingKey, Validation};

    match decode::<TokenClaims>(
        token,
        &DecodingKey::from_secret(settings.jwt.secret.as_bytes()),
        &Validation::default(),
    ) {
        Ok(decoded) => {
            request.extensions_mut().insert(decoded.claims);
            next.run(request).await
        }
        Err(_) => Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(Body::empty())
            .unwrap(),
    }
}