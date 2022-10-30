use std::error::Error;

use crate::model::social::SocialType::Telegram;
use crate::model::SocialCreateDto;
use crate::security::Claims;

use super::create;
use super::find_by_id;

#[sqlx_database_tester::test(pool(variable = "db"))]
async fn given_existing_id_when_find_by_id_then_found() -> Result<(), Box<dyn Error>> {
    //given
    let claims = Claims {
        sub: 11,
        company: "tomoru".to_owned(),
        exp: 2000000000,
    };
    let expected_social = create(&db, &claims, SocialCreateDto { social_type: Telegram }).await?;

    //when
    let actual_social = find_by_id(&db, expected_social.id).await?;

    //then
    assert_eq!(expected_social, actual_social, "should find correct social");
    Ok(())
}

#[sqlx_database_tester::test(pool(variable = "db"))]
async fn given_unknown_id_when_find_by_id_then_not_found() -> Result<(), Box<dyn Error>> {
    //given
    let unknown_id = 99999;

    //when
    let actual_social = find_by_id(&db, unknown_id).await;

    //then
    assert!(actual_social.is_err());
    Ok(())
}