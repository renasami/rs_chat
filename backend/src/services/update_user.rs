use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use bcrypt::{hash, DEFAULT_COST};
use serde::Deserialize;
use sqlx::PgPool;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum UserUpdateError {
    #[error("User not found")]
    UserNotFound,
    #[error("No columns to update")]
    NoColumnsToUpdate,
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Something went wrong")]
    SomethingWentWrong,
}
#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub user_id: Uuid,
    pub new_username: Option<String>,
    pub new_password: Option<String>,
}

pub async fn update_user(
    State(pool): State<Arc<PgPool>>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Response, StatusCode> {
    // 🔥 1. ユーザーの存在確認
    sqlx::query!(
        "SELECT user_id, username, password_hash FROM users WHERE user_id = $1",
        payload.user_id
    )
    .fetch_optional(pool.as_ref())
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)? // 💡 SQL エラーを `500` に変換
    .ok_or(StatusCode::NOT_FOUND)?;

    // 🔥 2. 更新するフィールドを動的に構築
    let mut set_clauses: Vec<String> = vec![];
    let mut params: Vec<(usize, String)> = vec![]; // (パラメータ位置, 値)

    let mut param_index = 1; // `user_id` は最後に追加するので `1` から開始

    if let Some(username) = payload.new_username {
        set_clauses.push(format!("username = ${}", param_index));
        params.push((param_index, username));
        param_index += 1;
    }

    if let Some(password) = payload.new_password {
        let hashed_password =
            hash(&password, DEFAULT_COST).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        set_clauses.push(format!("password_hash = ${}", param_index));
        params.push((param_index, hashed_password));
        param_index += 1;
    }

    // 更新するカラムがない場合はエラー
    if set_clauses.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    // 🔥 3. クエリを組み立てる
    let query_str = format!(
        "UPDATE users SET {} WHERE user_id = ${} RETURNING user_id, username",
        set_clauses.join(", "),
        param_index
    );

    // 🔥 4. クエリを作成し、動的にバインド
    let mut query = sqlx::query_as::<_, (Uuid, String)>(&query_str);

    for (_, value) in params {
        query = query.bind(value);
    }

    query = query.bind(payload.user_id); // 最後に `user_id` をバインド

    let updated_user = query
        .fetch_one(pool.as_ref())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((axum::http::StatusCode::OK, Json(updated_user)).into_response())
}
