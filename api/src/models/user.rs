use chrono::NaiveDateTime;

pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub salt: String,
    pub created_at: NaiveDateTime,
    pub last_login: Option<NaiveDateTime>,
}
