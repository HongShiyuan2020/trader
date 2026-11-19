use crate::{database::db::get_pool, model::entity::entity::Entity};

#[derive(Debug, sqlx::FromRow)]
pub struct UserPlan {
    pub id: i64,
    pub user_id: i64,
    pub plan_id: i64,
}

impl Entity for UserPlan {
    fn table_name() -> &'static str {
        "user_plan"
    }

    async fn init_table() -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS user_plan (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                user_id INTEGER NOT NULL,
                plan_id INTEGER NOT NULL
            );
            "#,
        )
        .execute(get_pool().await)
        .await?;
        Ok(())
    }

    async fn insert(user_plan: &mut Self) -> Result<(), sqlx::Error> {
        let insert_result = sqlx::query("INSERT INTO user_plan (user_id, plan_id) VALUES (?, ?)")
            .bind(user_plan.user_id)
            .bind(user_plan.plan_id)
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
