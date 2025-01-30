use std::sync::Arc;

use axum::{middleware, routing::post, Router};
use sqlx::PgPool;

use crate::middleware::auth_middleware::auth_middleware;
use crate::services::update_user::update_user;

pub fn user_routes() -> Router<Arc<PgPool>> {
    Router::new()
        .route("/update_user", post(update_user))
        .route_layer(middleware::from_fn(auth_middleware))
}
