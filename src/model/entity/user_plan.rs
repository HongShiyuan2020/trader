use crate::{database::db::get_pool, model::entity::entity::Entity};

#[derive(Debug, sqlx::FromRow)]
pub struct UserPlan {
    pub id: i64,
    pub uid: String,
    pub pid: String,
}

impl Entity for UserPlan {
    async fn init_table() -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS user_plan (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                uid TEXT NOT NULL,
                pid TEXT NOT NULL
            );
            "#,
        )
        .execute(get_pool().await)
        .await?;
        Ok(())
    }

    async fn insert(user_plan: &mut Self) -> Result<(), sqlx::Error> {
        let insert_result = sqlx::query("INSERT INTO user_plan (uid, pid) VALUES (?, ?)")
            .bind(&user_plan.uid)
            .bind(&user_plan.pid)
            .execute(get_pool().await)
            .await?;
        user_plan.id = insert_result.last_insert_rowid();
        Ok(())
    }

    async fn list_all() -> Result<Vec<Self>, sqlx::Error> {
        let user_plans = sqlx::query_as::<_, UserPlan>(
            r#"
                SELECT * FROM user_plan ORDER BY id DESC;
            "#,
        )
        .fetch_all(get_pool().await)
        .await?;

        Ok(user_plans)
    }
}
