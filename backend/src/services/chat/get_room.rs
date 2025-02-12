use std::sync::Arc;

use axum::{
    extract::State,
    response::{IntoResponse, Response},
    Json,
};
use reqwest::StatusCode;
use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::middleware::auth_middleware::AuthUser;

#[derive(Debug, Serialize)]
struct RoomResponse {
    pub room_id: Uuid,
    pub room_name: String,
}

pub async fn get_roms(
    State(pool): State<Arc<PgPool>>,
    _: AuthUser,
) -> Result<Response, StatusCode> {
    let rooms = sqlx::query_as!(
        RoomResponse,
        r#"
            SELECT
                room_id,
                room_name
            FROM rooms
        "#
    )
    .fetch_all(pool.as_ref())
    .await
    .map_err(|_| StatusCode::BAD_REQUEST);

    match rooms {
        Ok(rooms) => Ok((axum::http::StatusCode::OK, Json(rooms)).into_response()),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}
