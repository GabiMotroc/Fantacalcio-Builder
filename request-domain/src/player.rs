use serde::{Deserialize, Serialize};

pub struct Player {
    pub id: i32,
    pub fantacalcio_id: i32,
    pub position: Position,
    pub name: String,
    pub team: String,
    pub is_active: bool,
}

#[derive(Serialize, Deserialize, Debug)]
// #[sqlx(type_name = "position")]
pub enum Position {
    Goalkeeper,
    Defender,
    Midfielder,
    Forward,
}
