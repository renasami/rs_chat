use std::sync::Arc;

use axum::{
    extract::State,
    response::{IntoResponse, Response},
    Json,
};
use bcrypt::verify;
use sqlx::PgPool;
use uuid::Uuid;

use super::{
    auth_common::create_jwt,
    structs::{AuthRequest, AuthResponse},
};

pub async fn login_user(
    State(pool): State<Arc<PgPool>>,
    Json(payload): Json<AuthRequest>,
) -> Result<Response, axum::http::StatusCode> {
    let user = sqlx::query!(
        r#"
        SELECT user_id, username, password_hash
        FROM users
        WHERE username = $1
        "#,
        payload.username
    )
    .fetch_optional(pool.as_ref())
    .await
    .unwrap();

    if let Some(user) = user {
        let is_valid = verify(&payload.password, &user.password_hash).unwrap_or(false);
        if is_valid {
            let user_id = user.user_id;

            // 🔥 JWT を発行
            let access_token = create_jwt(user_id.to_string(), 15);
            let refresh_token = create_jwt(user_id.to_string(), 7 * 24 * 60); // 7日間

            sqlx::query!("DELETE FROM refresh_tokens WHERE user_id = $1", user_id)
                .execute(pool.as_ref())
                .await
                .unwrap();

            // 🔥 DB にリフレッシュトークンを保存
            sqlx::query!(
                "INSERT INTO refresh_tokens (id, user_id, token, expires_at) VALUES ($1, $2, $3, NOW() + interval '7 days')",
                Uuid::new_v4(), // ✅ `id` を明示的に設定
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

            return Ok((axum::http::StatusCode::OK, Json(response)).into_response());
        }
    }

    Err(axum::http::StatusCode::UNAUTHORIZED)
}
