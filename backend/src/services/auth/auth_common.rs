use std::env;

use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use uuid::Uuid;

use crate::middleware::auth::Claims;

pub fn create_jwt(user_id: String, expires_in_minutes: i64) -> String {
    let exp = Utc::now()
        .checked_add_signed(Duration::minutes(expires_in_minutes))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims { sub: user_id, exp };

    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .unwrap()
}

pub fn extract_user_id(
    auth_header: Option<TypedHeader<Authorization<Bearer>>>,
) -> Result<Uuid, axum::http::StatusCode> {
    if let Some(TypedHeader(Authorization(bearer))) = auth_header {
        let token = bearer.token();
        let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

        let decoded = jsonwebtoken::decode::<Claims>(
            token,
            &jsonwebtoken::DecodingKey::from_secret(secret.as_ref()),
            &jsonwebtoken::Validation::default(),
        );

        if let Ok(data) = decoded {
            return Ok(Uuid::parse_str(&data.claims.sub).unwrap());
        }
    }

    Err(axum::http::StatusCode::UNAUTHORIZED)
}
