use std::sync::Arc;

use axum::{routing::post, Router};
use sqlx::PgPool;

use crate::services::update_user::update_user;

pub fn user_routes() -> Router<Arc<PgPool>> {
    Router::new().route("/update_user", post(update_user))
}
