use crate::{database::db::get_pool, model::entity::entity::Entity};

#[derive(Debug, sqlx::FromRow)]
pub struct PlanNode {
    pub id: i64,
    pub plan_id: i64,
    pub node_id: i64,
}

impl Entity for PlanNode {
    fn table_name() -> &'static str {
        "plan_node"
    }

    async fn init_table() -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS plan_node (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                plan_id INTEGER NOT NULL,
                node_id INTEGER NOT NULL
            );
            "#,
        )
        .execute(get_pool().await)
        .await?;
        Ok(())
    }

    async fn insert(plan_node: &mut Self) -> Result<(), sqlx::Error> {
        let insert_result = sqlx::query("INSERT INTO plan_node (plan_id, node_id) VALUES (?, ?)")
            .bind(plan_node.plan_id)
            .bind(plan_node.node_id)
            .execute(get_pool().await)
            .await?;
        plan_node.id = insert_result.last_insert_rowid();
        Ok(())
    }

    async fn list_all() -> Result<Vec<Self>, sqlx::Error> {
        let plan_nodes = sqlx::query_as::<_, PlanNode>(
            r#"
                SELECT * FROM plan_node ORDER BY id DESC;
            "#,
        )
        .fetch_all(get_pool().await)
        .await?;

        Ok(plan_nodes)
    }
}
