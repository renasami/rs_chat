use std::sync::Arc;

use crate::services::{self};
use axum::{routing::post, Router};
use sqlx::PgPool;

use services::{login::login_user, logout::logout, register::register_user};

// ルーターを設定
pub fn auth_routes() -> Router<Arc<PgPool>> {
    Router::new()
        .route("/register", post(register_user))
        .route("/login", post(login_user))
        .route("/logout", post(logout))
}
