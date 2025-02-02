use std::sync::Arc;

use axum::{
    extract::State,
    response::{IntoResponse, Response},
    Json,
};
use bcrypt::{hash, DEFAULT_COST};
use sqlx::PgPool;
use uuid::Uuid;

use super::{
    auth_common::create_jwt,
    structs::{AuthRequest, AuthResponse},
};

pub async fn register_user(
    State(pool): State<Arc<PgPool>>,
    Json(payload): Json<AuthRequest>,
) -> Result<Response, axum::http::StatusCode> {
    let hashed_password = match hash(&payload.password, DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => {
            return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let user = sqlx::query!(
        r#"
        INSERT INTO users (user_id, username, password_hash, created_at)
        VALUES ($1, $2, $3, NOW())
        RETURNING user_id, username
        "#,
        Uuid::new_v4(),
        payload.username,
        hashed_password,
    )
    .fetch_one(pool.as_ref())
    .await;

    match user {
        Ok(user) => {
            let user_id = user.user_id;

            // 🔥 JWT アクセストークンを発行
            let access_token = create_jwt(user_id.to_string(), 15);
            let refresh_token = create_jwt(user_id.to_string(), 7 * 24 * 60); // 7日間

            // 🔥 DB にリフレッシュトークンを保存
            sqlx::query!(
                "INSERT INTO refresh_tokens (id, user_id, token, expires_at) VALUES ($1, $2, $3, NOW() + interval '7 days')",
                Uuid::new_v4(),
                user_id,
                refresh_token
            )
            .execute(pool.as_ref())
            .await
            .unwrap();

            let response = AuthResponse {
                user_id,
                username: user.username,
                access_token,
                refresh_token,
            };

            Ok((axum::http::StatusCode::CREATED, Json(response)).into_response())
            // ✅ `Response` 型を明示的に指定
        }
        Err(e) => {
            eprintln!("Database error: {:?}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
