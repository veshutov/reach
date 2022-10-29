use sqlx::{Error, Pool, Postgres};
use sqlx::postgres::PgPoolOptions;

pub type Db = Pool<Postgres>;

const PG_HOST: &str = "localhost";
const PG_ROOT_DB: &str = "reach";
const PG_ROOT_USER: &str = "veshutov";
const PG_ROOT_PWD: &str = "";

pub async fn init_db() -> Result<Db, Error> {
    let pool = new_db_pool(PG_HOST, PG_ROOT_DB, PG_ROOT_USER, PG_ROOT_PWD, 1)
        .await?;
    sqlx::migrate!()
        .run(&pool)
        .await?;
    Ok(pool)
}

async fn new_db_pool(host: &str, db: &str, user: &str, pwd: &str, max_con: u32) -> Result<Db, Error> {
    let con_string = format!("postgres://{}:{}@{}/{}", user, pwd, host, db);
    PgPoolOptions::new()
        .max_connections(max_con)
        .connect(&con_string)
        .await
}

#[cfg(test)]
#[path = "../_tests/model_db.rs"]
mod tests;