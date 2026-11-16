use crate::database::db::get_pool;


#[derive(Debug, sqlx::FromRow)]
pub struct User {
    id: i64,
    name: String,
    uid: String,
    password: String,
}

impl User {
    pub async fn init_db() {
        let _ = sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS user (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                uid TEXT NOT NULL,
                password TEXT NOT NULL,
            );
            "#
        ).execute(get_pool().await).await;
    }
}

