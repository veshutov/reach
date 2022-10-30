use sqlx::{Error, Pool, Postgres};
use sqlx::postgres::PgPoolOptions;

pub type Db = Pool<Postgres>;

pub async fn init_db() -> Result<Db, Error> {
    let pool = new_db_pool()
        .await?;
    sqlx::migrate!()
        .run(&pool)
        .await?;
    Ok(pool)
}

async fn new_db_pool() -> Result<Db, Error> {
    let con_string = std::env::var("DATABASE_URL")
        .expect("env var DATABASE_URL must be set");
    let pool_size = std::env::var("PG_POOL_SIZE")
        .expect("env var PG_POOL_SIZE must be set")
        .parse::<u32>()
        .expect("invalid pool size value");
    PgPoolOptions::new()
        .max_connections(pool_size)
        .connect(&con_string)
        .await
}

#[cfg(test)]
#[path = "../_tests/model_db.rs"]
mod tests;