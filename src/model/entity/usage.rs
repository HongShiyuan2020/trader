use crate::{database::db::get_pool, model::entity::entity::Entity};

#[derive(Debug, sqlx::FromRow)]
pub struct Usage {
    pub id: i64,
    pub uid: String,
    pub nid: String,
    pub usage: i64,
}

impl Entity for Usage {
    async fn init_table() -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS usage (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                uid TEXT NOT NULL,
                nid TEXT NOT NULL,
                usage INTEGER NOT NULL
            );
            "#,
        )
        .execute(get_pool().await)
        .await?;
        Ok(())
    }

    async fn insert(usage: &mut Self) -> Result<(), sqlx::Error> {
        let insert_result = sqlx::query("INSERT INTO usage (uid, nid, usage) VALUES (?, ?, ?)")
            .bind(&usage.uid)
            .bind(&usage.nid)
            .bind(usage.usage)
            .execute(get_pool().await)
            .await?;
        usage.id = insert_result.last_insert_rowid();
        Ok(())
    }

    async fn list_all() -> Result<Vec<Self>, sqlx::Error> {
        let usages = sqlx::query_as::<_, Usage>(
            r#"
                SELECT * FROM usage ORDER BY id DESC;
            "#,
        )
        .fetch_all(get_pool().await)
        .await?;

        Ok(usages)
    }
}
