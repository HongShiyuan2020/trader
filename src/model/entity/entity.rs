use sqlx::AssertSqlSafe;

use crate::database::db::get_pool;

pub trait Entity: Sized {
    fn table_name() -> &'static str;
    async fn init_table() -> Result<(), sqlx::Error>;
    async fn insert(entity: &mut Self) -> Result<(), sqlx::Error>;
    async fn list_all() -> Result<Vec<Self>, sqlx::Error>;
    async fn count() -> Result<i64, sqlx::Error> {
        let sql = format!("SELECT count(*) FROM \"{}\";", Self::table_name());
        let count: i64 = sqlx::query_scalar(AssertSqlSafe(sql))
            .fetch_one(get_pool().await)
            .await?;
        return Ok(count);
    }
}
