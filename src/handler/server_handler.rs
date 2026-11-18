use axum::{Json, http::StatusCode};
use tracing::info;

use crate::model::{dto::server::ServerCreateQuery, entity::{entity::Entity, server::Server, user::User}, extension::current_user::CurrentUser};

pub async fn create(
    user: CurrentUser,
    Json(payload): Json<ServerCreateQuery>
) -> (StatusCode, String) {

    info!("{}", user.uid);

    info!("{:#?}", payload);

    let mut record = Server {
        id: 0,
        name: payload.name,
        sid: "random-uid".to_string(),
        ip: payload.ip,
        domain: None,
        ssh_port: 22,
        ssh_user: payload.ssh_user,
        ssh_password: payload.ssh_password,
    };
    Server::insert(&mut record).await.expect("");

    (StatusCode::CREATED, String::from("Created OK"))
}

pub async fn health_check() {
    
}

pub async fn page() {
    
}
