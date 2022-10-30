use sqlx::{Error, Pool, Postgres};
use sqlx::postgres::PgPoolOptions;

pub type Db = Pool<Postgres>;

pub async fn init_db() -> Result<Db, Error> {
    let pool = new_db_pool(10)
        .await?;
    sqlx::migrate!()
        .run(&pool)
        .await?;
    Ok(pool)
}

async fn new_db_pool(max_con: u32) -> Result<Db, Error> {
    let con_string = std::env::var("DATABASE_URL").expect("env var DATABASE_URL must be set");
    PgPoolOptions::new()
        .max_connections(max_con)
        .connect(&con_string)
        .await
}

#[cfg(test)]
#[path = "../_tests/model_db.rs"]
mod tests;