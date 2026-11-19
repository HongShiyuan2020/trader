use serde::{Deserialize, Serialize};

use crate::model::entity::server::Server;

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerCreateQuery {
    pub name: String,
    pub ip: String,
    pub domain: Option<String>,
    pub ssh_port: u16,
    pub ssh_user: String,
    pub ssh_password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerCreateAns {
    pub id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct ServerPageQuery {
    pub page: i64,
    pub size: i64,
}

impl Default for ServerPageQuery {
    fn default() -> Self {
        Self {
            page: 1,
            size: 10,
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct ServerPageAns {
    pub id: i64,
    pub name: String,
    pub ip: String,
    pub domain: Option<String>,
    pub ssh_port: u16,
    pub ssh_user: String,
    pub ssh_password: String,
}
