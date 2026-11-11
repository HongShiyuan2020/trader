use axum::{Json, Router, routing::{get, post}};
use serde::Serialize;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Serialize)]
struct User {
    name: String,
    passwd: String,
    total: u64,
    used: u64,
}

#[tokio::main]
async fn main() {

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();
        
    info!("Application is starting...");

    let app = Router::new()
        .route("/health", get(|| async {
            "Health OK!"
        }))
        .route("/health", post(|| async {
            "Health OK! \n Answer Post!"
        }))
        .route("/user", get(|| async {
            Json(User {
                name: String::from("syhong"),
                passwd: String::from("123"),
                total: 1024,
                used: 2,
            })
        }));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8001")
        .await
        .unwrap();

    println!("Server is running at http://127.0.0.1:8001");
  
    axum::serve(listener, app).await.unwrap();

} 
