use crate::{database::db::get_pool, model::entity::entity::Entity};

#[derive(Debug, sqlx::FromRow)]
pub struct Node {
    pub id: i64,
    pub name: String,
    pub nid: String,
    pub sid: String,
    pub node_type: String,
}

impl Entity for Node {
    async fn init_table() -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS node (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                nid TEXT NOT NULL,
                sid TEXT NOT NULL,
                node_type TEXT NOT NULL
            );
            "#,
        )
        .execute(get_pool().await)
        .await?;
        Ok(())
    }

    async fn insert(node: &mut Self) -> Result<(), sqlx::Error> {
        let insert_result = sqlx::query(
            "INSERT INTO node (name, nid, sid, node_type) VALUES (?, ?, ?, ?)",
        )
        .bind(&node.name)
        .bind(&node.nid)
        .bind(&node.sid)
        .bind(&node.node_type)
        .execute(get_pool().await)
        .await?;
        node.id = insert_result.last_insert_rowid();
        Ok(())
    }

    async fn list_all() -> Result<Vec<Self>, sqlx::Error> {
        let nodes = sqlx::query_as::<_, Node>(
            r#"
                SELECT * FROM node ORDER BY id DESC;
            "#,
        )
        .fetch_all(get_pool().await)
        .await?;

        Ok(nodes)
    }
}
