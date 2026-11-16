#[derive(Debug, sqlx::FromRow)]
pub struct PlanNode {
    id: i64,
    pid: String,
    nid: String,
}
