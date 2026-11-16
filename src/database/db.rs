use std::{str::FromStr};

use sqlx::{
    Pool, Sqlite,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};
use tokio::sync::OnceCell;

async fn create_sqlite_pool() -> Pool<Sqlite> {
    let options = SqliteConnectOptions::from_str("sqlite:trader.db")
        .unwrap()
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await
        .unwrap();

    return pool;
}

static GLOBAL_DB_POOL: OnceCell<Pool<Sqlite>> = OnceCell::const_new();

pub async fn get_pool() -> &'static Pool<Sqlite> {
    GLOBAL_DB_POOL
        .get_or_init(|| async { create_sqlite_pool().await })
        .await
}
