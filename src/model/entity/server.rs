use crate::{database::db::get_pool, model::entity::entity::Entity};

#[derive(Debug, sqlx::FromRow)]
pub struct Server {
    pub id: i64,
    pub name: String,
    pub ip: String,
    pub domain: Option<String>,
    pub ssh_port: u16,
    pub ssh_user: String,
    pub ssh_password: String,
}
impl Server {
    pub async fn page(
        params: &crate::model::dto::server::ServerPageQuery,
    ) -> Result<Vec<Self>, sqlx::Error> {
        let servers = sqlx::query_as::<_, Self>(
            r#"
            SELECT * FROM server ORDER BY id DESC LIMIT ? OFFSET ?;
            "#,
        )
        .bind(params.size)
        .bind((params.page - 1) * params.size)
        .fetch_all(get_pool().await)
        .await?;
        Ok(servers)
    }

    pub async fn count() -> Result<i64, sqlx::Error> {
        let count: i64 = sqlx::query_scalar(
            r#"
            SELECT count(id) FROM server;
            "#,
        )
        .fetch_one(get_pool().await)
        .await?;

        Ok(count)
    }
}

impl Entity for Server {
    fn table_name() -> &'static str {
        "server"
    }

    async fn init_table() -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS server (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                ip TEXT NOT NULL,
                domain TEXT,
                ssh_port INTEGER NOT NULL,
                ssh_user TEXT NOT NULL,
                ssh_password TEXT NOT NULL
            );
            "#,
        )
        .execute(get_pool().await)
        .await?;
        Ok(())
    }

    async fn insert(server: &mut Self) -> Result<(), sqlx::Error> {
        let insert_result = sqlx::query(
            "INSERT INTO server (name, ip, domain, ssh_port, ssh_user, ssh_password) VALUES (?, ?, ?, ?, ?, ?)",
        )
        .bind(&server.name)
        .bind(&server.ip)
        .bind(&server.domain)
        .bind(server.ssh_port)
        .bind(&server.ssh_user)
        .bind(&server.ssh_password)
        .execute(get_pool().await)
        .await?;
        server.id = insert_result.last_insert_rowid();
        Ok(())
    }

    async fn list_all() -> Result<Vec<Self>, sqlx::Error> {
        let servers = sqlx::query_as::<_, Server>(
            r#"
                SELECT * FROM server ORDER BY id DESC;
            "#,
        )
        .fetch_all(get_pool().await)
        .await?;

        Ok(servers)
    }
}
