use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

pub async fn init_database_pool() -> Pool<Postgres> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅  Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥  Failed to connect to the database, {:?}: {:?}", database_url, err);
            std::process::exit(2);
        }
    }
}
