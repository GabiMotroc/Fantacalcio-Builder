use chrono::Utc;
use sqlx::PgPool;

use request_domain::player::Player;

use crate::models::player::PlayerEntity;
// use crate::models::player::Player;
use crate::models::user::User;

pub struct Db;

impl Db {
    pub async fn create_user(email: &str, password: &str, salt: &str, pool: &PgPool) -> Result<i32, sqlx::Error> {
        let user_id = sqlx::query!(
            "INSERT INTO users (email, password, salt, created_at) VALUES (lower($1), $2, $3, $4) RETURNING id",
            email, password, salt, Utc::now().naive_utc())
            .map(|row| row.id)
            .fetch_one(pool)
            .await?;

        Ok(user_id)
    }
    pub async fn get_user(email: &str, pool: &PgPool) -> Result<User, sqlx::Error> {
        sqlx::query_as!(
            User,
            "select * from users where email = $1",
            email)
            .fetch_one(pool)
            .await
    }

    pub async fn get_players(pool: &PgPool) -> Result<Vec<PlayerEntity>, sqlx::Error> {
        sqlx::query_as!(
            PlayerEntity,
            r#"select id, fantacalcio_id, name, team, is_active, position as "position: _" from players limit 25"#
        )
            .fetch_all(pool)
            .await
    }
}