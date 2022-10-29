use std::error::Error;

use crate::model::db::init_db;
use crate::model::social::SocialType::Telegram;
use crate::security::utx_from_token;

use super::SocialDao;

#[tokio::test]
async fn model_social_find_by_id() -> Result<(), Box<dyn Error>> {
    let db = init_db().await?;
    let social = SocialDao::find_by_id(&db, 1).await?;

    assert_eq!(1, social.id, "social id");
    assert_eq!(Telegram, social.social_type, "social type");

    Ok(())
}