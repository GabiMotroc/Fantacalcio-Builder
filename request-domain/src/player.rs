use std::fmt;
use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::player::Position::{Defender, Forward, Goalkeeper, Midfielder};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Player {
    pub id: i32,
    pub fantacalcio_id: i32,
    pub position: Position,
    pub name: String,
    pub team: String,
    pub is_active: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
// #[sqlx(type_name = "position")]
pub enum Position {
    Goalkeeper,
    Defender,
    Midfielder,
    Forward,
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl Position {
    pub fn all() -> Vec<Position> {
        vec![Goalkeeper, Defender, Midfielder, Forward]
    }
}
