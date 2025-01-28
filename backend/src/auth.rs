use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use thiserror::Error;
use tracing::error; // 🔥 エラーログ用

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // ユーザーID
    pub exp: usize,  // 有効期限
}

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Invalid token: {field1}")]
    InvalidToken { field1: String },
}

pub fn create_jwt(user_id: &str) -> String {
    let exp = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        exp,
    };

    let secret = match env::var("JWT_SECRET") {
        Ok(secret) => secret,
        Err(_) => {
            error!("JWT_SECRET is not set in the environment variables");
            panic!("JWT_SECRET must be set");
        }
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .unwrap_or_else(|e| {
        error!("Failed to create JWT: {:?}", e);
        panic!("Failed to create JWT");
    })
}

pub fn validate_jwt(token: &str) -> Result<String, AuthError> {
    let secret = match env::var("JWT_SECRET") {
        Ok(secret) => secret,
        Err(_) => {
            error!("JWT_SECRET is not set in the environment variables");
            return Err(AuthError::InvalidToken {
                field1: "JWT_SECRET is missing".to_string(),
            });
        }
    };

    match decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    ) {
        Ok(decoded) => Ok(decoded.claims.sub),
        Err(e) => {
            error!("JWT validation failed: {:?}", e);
            Err(AuthError::InvalidToken {
                field1: format!("JWT validation error: {:?}", e),
            })
        }
    }
}
