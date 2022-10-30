use std::error::Error;

use super::init_db;

#[tokio::test]
async fn model_db_init_db() -> Result<(), Box<dyn Error>> {
    init_db().await?;

    Ok(())
}