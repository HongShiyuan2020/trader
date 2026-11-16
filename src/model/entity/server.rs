#[derive(Debug, sqlx::FromRow)]
pub struct Server {
    id: i64,
    name: String,
    sid: String,
    ip: String,
    domain: String,
    ssh_port: u16,
    ssh_user: String,
    ssh_password: String,
}
