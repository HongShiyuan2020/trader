use axum::{Json, extract::Query, http::StatusCode};
use tracing::info;

use crate::model::{
    dto::{
        page::Page,
        server::{ServerCreateAns, ServerCreateQuery, ServerPageAns, ServerPageQuery},
    },
    entity::{entity::Entity, server::Server, user::User},
    extension::current_user::CurrentUser,
};

pub async fn create(
    user: CurrentUser,
    Json(payload): Json<ServerCreateQuery>,
) -> (StatusCode, Json<ServerCreateAns>) {
    let mut record = Server {
        id: 0,
        name: payload.name,
        ip: payload.ip,
        domain: None,
        ssh_port: 22,
        ssh_user: payload.ssh_user,
        ssh_password: payload.ssh_password,
    };
    Server::insert(&mut record).await.expect("");

    (StatusCode::CREATED, Json(ServerCreateAns { id: record.id }))
}

pub async fn health_check() {}

pub async fn page(
    user: CurrentUser,
    Query(params): Query<ServerPageQuery>,
) -> (StatusCode, Json<Page<ServerPageAns>>) {
    (
        StatusCode::OK,
        Json(Page {
            data: Server::page(&params)
                .await
                .expect("")
                .into_iter()
                .map(|e| ServerPageAns {
                    id: e.id,
                    name: e.name,
                    ip: e.ip,
                    domain: e.domain,
                    ssh_port: e.ssh_port,
                    ssh_user: e.ssh_user,
                    ssh_password: e.ssh_password,
                })
                .collect(),
            total: Server::count().await.expect(""),
            page: params.page,
            page_size: params.size,
        }),
    )
}
