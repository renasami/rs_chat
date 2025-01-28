use axum::{
    extract::State,
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};
use bcrypt::{hash, DEFAULT_COST};
use chrono::{Duration, NaiveDateTime, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use sqlx::{
    types::time::{OffsetDateTime, PrimitiveDateTime},
    Encode, PgPool,
};
use std::env;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub user_id: Uuid,
    pub username: String,
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub async fn register_user(
    State(pool): State<Arc<PgPool>>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Response, axum::http::StatusCode> {
    // ✅ 戻り値を `Result<Response, StatusCode>` に変更
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

// 🔥 JWT を作成する関数
fn create_jwt(user_id: String, expires_in_minutes: i64) -> String {
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

// ルーターを設定
pub fn auth_routes() -> Router<Arc<PgPool>> {
    Router::new().route("/register", post(register_user))
}
