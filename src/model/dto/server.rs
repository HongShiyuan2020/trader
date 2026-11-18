use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct ServerCreateQuery {
   pub name: String,
   pub ip: String,
   pub domain: Option<String>,
   pub ssh_port: u16,
   pub ssh_user: String,
   pub ssh_password: String,
}
