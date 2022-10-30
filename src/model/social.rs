use serde::Deserialize;
use serde::Serialize;

use crate::model::db::Db;
use crate::model::RepoError;
use crate::security::Claims;

#[derive(sqlx::Type, Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[sqlx(type_name = "social_type")]
#[sqlx(rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum SocialType {
    Telegram
}

#[derive(sqlx::FromRow, Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Social {
    pub id: i64,
    pub social_type: SocialType,
}

#[derive(Debug, Deserialize)]
pub struct SocialCreateDto {
    pub social_type: SocialType,
}

pub async fn create(db: &Db, utx: &Claims, social: SocialCreateDto) -> Result<Social, RepoError> {
    let sql = "INSERT INTO social (social_type, created_at, created_by, updated_at, updated_by) VALUES ($1, now(), $2, now(), $2) returning id, social_type";
    let query = sqlx::query_as::<_, Social>(&sql)
        .bind(social.social_type)
        .bind(utx.sub);

    let social = query.fetch_one(db).await?;

    Ok(social)
}

pub async fn find_by_id(db: &Db, id: i64) -> Result<Social, RepoError> {
    let sql = format!("SELECT * FROM social WHERE id = {}", id);
    let query = sqlx::query_as::<_, Social>(&sql);

    let social = query.fetch_one(db)
        .await.map_err(|sql_err| match sql_err {
        sqlx::Error::RowNotFound => RepoError::EntityNotFound("social", id.to_string()),
        other => RepoError::SqlxError(other)
    })?;

    Ok(social)
}

#[cfg(test)]
#[path = "../_tests/model_social.rs"]
mod tests;