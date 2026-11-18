use std::{env};

use axum::{
    Router,
    routing::{get, post},
};

use dotenvy::dotenv;
use tracing::{info};

use crate::{
    handler::server_handler,
    log::log::init_logger,
    model::entity::{
        entity::Entity, node::Node, plan::Plan, plan_node::PlanNode, server::Server, usage::Usage,
        user::User, user_plan::UserPlan,
    },
};

mod database;
mod handler;
mod log;
mod model;

#[tokio::main]
async fn main() {
    info!("Loading .env...");
    dotenv().ok();

    info!("Loggin init...");
    init_logger().await;

    info!("Database init...");
    User::init_table().await.unwrap();
    Server::init_table().await.unwrap();
    Plan::init_table().await.unwrap();
    Node::init_table().await.unwrap();
    Usage::init_table().await.unwrap();
    UserPlan::init_table().await.unwrap();
    PlanNode::init_table().await.unwrap();

    info!("Application is starting...");
    let ip = env::var("IP").expect("Please setting IP in .env!");
    let port: i32 = env::var("PORT")
        .unwrap()
        .parse()
        .expect("Please checking PORT in .env!");

    let app = Router::new()
        .route("/health", get(|| async { "Health OK!" }))
        .route("/health", post(|| async { "Health OK! \n Answer Post!" }))
        .route("/api/server/create", post(server_handler::create));

    let listener = tokio::net::TcpListener::bind(format!("{ip}:{port}"))
        .await
        .unwrap();

    info!("Server is running at http://{ip}:{port}");

    axum::serve(listener, app).await.unwrap();
}
