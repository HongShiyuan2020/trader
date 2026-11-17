pub trait Entity: Sized {
    async fn init_table() -> Result<(), sqlx::Error>;
    async fn insert(entity: &mut Self) -> Result<(), sqlx::Error>;
    async fn list_all() -> Result<Vec<Self>, sqlx::Error>;
}
