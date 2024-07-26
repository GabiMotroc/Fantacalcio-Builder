use chrono::Utc;
use sqlx::PgPool;

use crate::models::error::ApiError;
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
            r#"select id, fantacalcio_id, name, team, is_active, position as "position: _" from players"#
        )
            .fetch_all(pool)
            .await
    }

    pub async fn select_players(pool: &PgPool, user_id: i32, player_ids: Vec<i32>) -> Result<(), ApiError> {
        sqlx::query!(
            "insert into selected_players (user_id, player_id) select * from unnest($1::INT4[], $2::INT4[])", 
            &vec![user_id; player_ids.len()], &player_ids
        )
            .execute(pool)
            .await?;

        Ok(())
    }
}