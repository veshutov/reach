use std::error::Error;

use sqlx::migrate::Migrator;

use super::init_db;

#[tokio::test]
async fn model_db_init_db() -> Result<(), Box<dyn Error>> {
    let db = init_db().await?;

    let socials = sqlx::query("SELECT * FROM socials").fetch_all(&db).await?;
    assert_eq!(1, socials.len(), "number of socials");

    Ok(())
}