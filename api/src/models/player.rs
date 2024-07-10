use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow)]
#[derive(Serialize, Deserialize, Debug)]
// pub use PLayer as PlayerEntity;
pub struct PlayerEntity {
    pub id: i32,
    pub fantacalcio_id: i32,
    pub position: PositionEntity,
    pub name: String,
    pub team: String,
    pub is_active: bool,
}

#[derive(Serialize, Deserialize, Debug, sqlx::Type)]
#[sqlx(type_name = "position")]
pub enum PositionEntity {
    Goalkeeper,
    Defender,
    Midfielder,
    Forward,
}

