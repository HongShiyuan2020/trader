use crate::{database::db::get_pool, model::entity::entity::Entity};


#[derive(Debug, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub password: String,
}

impl Entity for User {
    fn table_name() -> &'static str {
        "user"
    }

    async fn init_table() -> Result<(), sqlx::Error>{
        let res = sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS user (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                password TEXT NOT NULL
            );
            "#
        ).execute(get_pool().await).await?;
        Ok(())
    }

    async fn insert(user: &mut Self) -> Result<(), sqlx::Error> {
        let insert_result = sqlx::query("INSERT INTO user (name, password) VALUES (?, ?)")
            .bind(&user.name)
            .bind(&user.password)
            .execute(get_pool().await)
            .await?;
        user.id = insert_result.last_insert_rowid();
        Ok(())
    }

    async fn list_all() -> Result<Vec<User>, sqlx::Error> {
        let users = sqlx::query_as::<_, User>(r#"
                SELECT * FROM user ORDER BY id DESC;
            "#).fetch_all(get_pool().await)
            .await?;

        Ok(users)
    }
}

