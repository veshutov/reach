use crate::model::db::Db;
use crate::model::Error;

#[derive(sqlx::Type, Debug, PartialEq)]
#[sqlx(type_name = "social_type")]
pub enum SocialType {
    Telegram
}

#[derive(sqlx::FromRow)]
pub struct Social {
    pub id: i64,
    pub social_type: SocialType,
}

pub struct SocialDao;

impl SocialDao {
    pub async fn find_by_id(db: &Db, id: i64) -> Result<Social, Error> {
        let sql = format!("SELECT * FROM social WHERE id = {}", id);
        let query = sqlx::query_as::<_, Social>(&sql);
        let social = query.fetch_one(db).await?;
        Ok(social)
    }
}

#[cfg(test)]
#[path = "../_tests/model_social.rs"]
mod tests;