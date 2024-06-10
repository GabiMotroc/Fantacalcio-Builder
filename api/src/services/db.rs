use chrono::Utc;
use sqlx::PgPool;

pub struct Db;

impl Db {
    pub async fn create_user(email: &str, password: &str, salt: &str, pool: &PgPool) -> Result<i32, sqlx::Error> {
        sqlx::query!(
            "INSERT INTO users (email, password, salt, created_at) VALUES (lower($1), $2, $3, $4) RETURNING id",
            email, password, salt, Utc::now().naive_utc())
            .map(|row| row.id)
            .fetch_one(pool)
            .await
    }
}