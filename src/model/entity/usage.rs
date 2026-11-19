use crate::{database::db::get_pool, model::entity::entity::Entity};

#[derive(Debug, sqlx::FromRow)]
pub struct Usage {
    pub id: i64,
    pub user_id: i64,
    pub node_id: i64,
    pub usage: i64,
}

impl Entity for Usage {
    fn table_name() -> &'static str {
        "usage"
    }

    async fn init_table() -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS usage (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                user_id INTEGER NOT NULL,
                node_id INTEGER NOT NULL,
                usage INTEGER NOT NULL
            );
            "#,
        )
        .execute(get_pool().await)
        .await?;
        Ok(())
    }

    async fn insert(usage: &mut Self) -> Result<(), sqlx::Error> {
        let insert_result = sqlx::query("INSERT INTO usage (user_id, node_id, usage) VALUES (?, ?, ?)")
            .bind(usage.user_id)
            .bind(usage.node_id)
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
