#[derive(Debug, sqlx::FromRow)]
pub struct Plan {
    id: i64,
    name: String,
    pid: String,
    total: i64,
}
