use std::sync::Arc;

use axum::routing::get;
use axum::{middleware, routing::post, Router};
use sqlx::PgPool;

use crate::middleware::auth_middleware::auth_middleware;
use crate::services::auth::me::get_me;
use crate::services::chat::create_room::create_room;
use crate::services::update_user::update_user;

pub fn user_routes() -> Router<Arc<PgPool>> {
    Router::new()
        .route("/update_user", post(update_user))
        .route("/create_room", post(create_room))
        .route("/get_rooms")
        .route("/me", get(get_me))
        .route_layer(middleware::from_fn(auth_middleware))
}
