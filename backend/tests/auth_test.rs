use backend::main::start_server; // 🔥 サーバーを起動する関数を `main.rs` からインポート
use reqwest::Client;
use std::sync::Once;
use tokio;

const BASE_URL: &str = "http://localhost:3000";

// テスト用に一度だけサーバーを起動する
static INIT: Once = Once::new();

async fn setup() {
    INIT.call_once(|| {
        tokio::spawn(async {
            start_server().await.unwrap(); // `main.rs` から `start_server()` を呼び出す
        });
    });

    // サーバーが立ち上がるまで待機
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
}

#[tokio::test]
async fn test_jwt_auth_flow() {
    setup().await; // サーバーを起動

    let client = Client::new();

    // 1️⃣ `/login` で JWT を取得
    let login_response = client
        .post(format!("{}/login", BASE_URL))
        .send()
        .await
        .expect("Failed to send login request");

    assert!(login_response.status().is_success());

    let jwt: String = login_response.text().await.expect("Failed to extract JWT");

    println!("✅ JWT: {}", jwt);

    // 2️⃣ `/protected` に JWT を送信
    let protected_response = client
        .get(format!("{}/protected", BASE_URL))
        .header("Authorization", format!("Bearer {}", jwt))
        .send()
        .await
        .expect("Failed to send protected request");

    assert!(protected_response.status().is_success());

    let protected_text = protected_response
        .text()
        .await
        .expect("Failed to extract protected response");
    println!("✅ Protected Response: {}", protected_text);
}
