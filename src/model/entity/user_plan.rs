#[derive(Debug, sqlx::FromRow)]
pub struct UserPlan {
    id: i64,
    uid: String,
    pid: String,
}
