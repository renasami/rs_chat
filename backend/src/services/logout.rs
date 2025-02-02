use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use axum_extra::headers::{authorization::Bearer, Authorization};
use axum_extra::TypedHeader;
use serde::Serialize;
use sqlx::PgPool;
use std::sync::Arc;
use tracing::error;

#[derive(Serialize)]
struct LogoutResponse {
    message: String,
}

/// ログアウトエンドポイント
pub async fn logout(
    State(pool): State<Arc<PgPool>>,
    auth_header: Option<TypedHeader<Authorization<Bearer>>>,
) -> Response {
    // Authorization ヘッダーのチェック
    let Some(auth_value) = auth_header else {
        error!("Missing Authorization header");
        return (StatusCode::UNAUTHORIZED, "Missing Authorization header").into_response();
    };

    let token = auth_value.token();

    // トークンをDBから削除
    let result = sqlx::query!("DELETE FROM refresh_tokens WHERE token = $1", token)
        .execute(pool.as_ref())
        .await;

    if let Err(err) = result {
        error!("Failed to delete refresh token: {:?}", err);
        return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to log out").into_response();
    }

    let response = LogoutResponse {
        message: "Successfully logged out".to_string(),
    };

    (StatusCode::OK, Json(response)).into_response()
}
