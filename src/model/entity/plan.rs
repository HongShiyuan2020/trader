use crate::{database::db::get_pool, model::entity::entity::Entity};

#[derive(Debug, sqlx::FromRow)]
pub struct Plan {
    pub id: i64,
    pub name: String,
    pub pid: String,
    pub total: i64,
}

impl Entity for Plan {
    async fn init_table() -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS plan (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                pid TEXT NOT NULL,
                total INTEGER NOT NULL
            );
            "#,
        )
        .execute(get_pool().await)
        .await?;
        Ok(())
    }

    async fn insert(plan: &mut Self) -> Result<(), sqlx::Error> {
        let insert_result = sqlx::query("INSERT INTO plan (name, pid, total) VALUES (?, ?, ?)")
            .bind(&plan.name)
            .bind(&plan.pid)
            .bind(plan.total)
            .execute(get_pool().await)
            .await?;
        plan.id = insert_result.last_insert_rowid();
        Ok(())
    }

    async fn list_all() -> Result<Vec<Self>, sqlx::Error> {
        let plans = sqlx::query_as::<_, Plan>(
            r#"
                SELECT * FROM plan ORDER BY id DESC;
            "#,
        )
        .fetch_all(get_pool().await)
        .await?;

        Ok(plans)
    }
}

