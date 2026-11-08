use std::string;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    id: u64,
    name: string,
    passwd: string,
    usage: i64,
}
