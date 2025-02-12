use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateRoomRequest {
    pub room_name: String,
    pub created_by: Uuid,
}

#[derive(Debug, Serialize)]
struct CreateRoomResponse {
    pub room_name: String,
}

pub async fn create_room(
    State(pool): State<Arc<PgPool>>,
    Json(payload): Json<CreateRoomRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    // ✅ `impl IntoResponse` を使う
    sqlx::query!(
        r#"
        INSERT INTO rooms (room_id, room_name, belongs, created_by, created_at) 
        VALUES ($1, $2, $3, $4, now())
        RETURNING room_name
        "#,
        Uuid::new_v4(),
        payload.room_name,
        &[payload.created_by], // ✅ `&[Uuid]` の形で渡す
        payload.created_by,
    )
    .fetch_one(pool.as_ref())
    .await
    .map_err(|_| StatusCode::BAD_REQUEST)?;

    Ok((
        StatusCode::CREATED,
        Json(CreateRoomResponse {
            room_name: payload.room_name,
        }),
    )) // ✅ `.into_response()` は不要
}
