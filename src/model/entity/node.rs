#[derive(Debug, sqlx::FromRow)]
pub struct Node {
    id: i64,
    name: String,
    nid: String,
    sid: String,
    node_type: String,
}
