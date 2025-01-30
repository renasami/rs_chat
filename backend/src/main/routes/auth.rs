use std::sync::Arc;

use axum::{routing::post, Router};
use sqlx::PgPool;

use crate::services::{login::login_user, register::register_user};

// ルーターを設定
pub fn auth_routes() -> Router<Arc<PgPool>> {
    Router::new()
        .route("/register", post(register_user))
        .route("/login", post(login_user))
}
