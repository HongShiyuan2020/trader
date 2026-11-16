use std::str::FromStr;

use axum::{
    Json, Router,
    routing::{get, post},
};

use tracing::info;

use crate::{log::log::init_logger, model::entity::user::User};

mod database;
mod model;
mod log;

#[tokio::main]
async fn main() {
    info!("Loggin init...");
    init_logger().await;

    info!("Loading .env...");
    

    info!("Database init...");
    User::init_db().await;
    
    info!("Application is starting...");

    let app = Router::new()
        .route("/health", get(|| async { "Health OK!" }))
        .route("/health", post(|| async { "Health OK! \n Answer Post!" }));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8001")
        .await
        .unwrap();

    info!("Server is running at http://127.0.0.1:8001");

    axum::serve(listener, app).await.unwrap();
}
