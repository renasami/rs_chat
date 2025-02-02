use std::{str::FromStr, sync::Arc};

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::middleware::auth_middleware::AuthUser;

#[derive(Debug, serde::Serialize)]
pub struct UserResponse {
    pub user_id: Uuid,
    pub username: String,
}

// 🔥 `/me` エンドポイント: 認証済みユーザー情報を取得
pub async fn get_me(
    State(pool): State<Arc<PgPool>>,
    payload: AuthUser, // Middleware で JWT 認証を通過
) -> Result<Response, StatusCode> {
    let user = sqlx::query_as!(
        UserResponse,
        "SELECT user_id, username FROM users WHERE user_id = $1",
        &payload.0
    )
    .fetch_optional(pool.as_ref())
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match user {
        Some(user) => Ok((axum::http::StatusCode::OK, Json(user)).into_response()),
        None => Err(StatusCode::NOT_FOUND),
    }
}
