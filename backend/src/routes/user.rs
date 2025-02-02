use std::sync::Arc;

use axum::routing::get;
use axum::{middleware, routing::post, Router};
use sqlx::PgPool;

use crate::middleware::auth_middleware::auth_middleware;
use crate::services::me::get_me;
use crate::services::update_user::update_user;

pub fn user_routes() -> Router<Arc<PgPool>> {
    Router::new()
        .route("/update_user", post(update_user))
        .route("/me", get(get_me))
        .route_layer(middleware::from_fn(auth_middleware))
}
