#[derive(Debug, sqlx::FromRow)]
pub struct Usage {
    id: i64,
    uid: String,
    nid: String,
    usage: i64,
}
