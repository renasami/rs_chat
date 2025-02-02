use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        ConnectInfo,
    },
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use backend::routes;
use dotenvy::dotenv;
use routes::{auth::auth_routes, user::user_routes};
use sqlx::PgPool;
use std::{env, net::SocketAddr};
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

async fn ws_handler(
    ws: WebSocketUpgrade,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    println!("Client {} connected", addr);
    ws.on_upgrade(move |socket| handle_socket(socket, addr))
}

async fn handle_socket(mut socket: WebSocket, addr: SocketAddr) {
    while let Some(Ok(msg)) = socket.recv().await {
        if let Message::Text(text) = msg {
            println!("Received from {}: {}", addr, text);
            if socket.send(Message::Text(text)).await.is_err() {
                println!("Failed to send message to {}", addr);
                return;
            }
        }
    }
    println!("Client {} disconnected", addr);
}

pub async fn start_server() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to the database");

    // CORS設定の作成
    let cors = CorsLayer::new().allow_origin(Any).allow_methods(Any);

    let app = Router::new()
        .merge(auth_routes())
        .merge(user_routes())
        .route("/ws", get(ws_handler))
        .route("/", get(handler))
        .with_state(pool.into()) // データベース接続プールを状態として追加
        .layer(cors); // CORS設定をレイヤーとして追加

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;

    Ok(())
}

#[tokio::main]
pub async fn main() {
    start_server().await.unwrap();
}

async fn handler() -> &'static str {
    "Hello, Rust Chat!"
}
