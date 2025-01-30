use crate::middleware::auth::{validate_jwt, AuthError};
use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
};
use tracing::error;

#[derive(Clone, Debug)]
pub struct AuthUser(pub String); // ユーザーIDを格納

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        // Authorization ヘッダーの取得
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|value| value.to_str().ok());

        // Authorization ヘッダーがあるか確認
        let Some(auth_value) = auth_header else {
            error!("Authorization header is missing");
            return Err((StatusCode::UNAUTHORIZED, "Missing Authorization header").into_response());
        };

        // "Bearer " プレフィックスの確認
        let Some(token) = auth_value.strip_prefix("Bearer ") else {
            error!("Invalid Authorization format");
            return Err((StatusCode::UNAUTHORIZED, "Invalid Authorization format").into_response());
        };

        // JWT 検証
        match validate_jwt(token) {
            Ok(user_id) => Ok(AuthUser(user_id)),
            Err(AuthError::InvalidToken { field1 }) => {
                error!("Invalid JWT token {}", field1);
                Err((
                    StatusCode::UNAUTHORIZED,
                    format!("Invalid JWT token: {}", field1),
                )
                    .into_response())
            }
        }
    }
}
