use chrono::{DateTime, Utc};

pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub last_login: DateTime<Utc>,
}
