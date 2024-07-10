use axum::extract::State;

use crate::models::app_state::AppState;
use crate::models::error::ApiError;
use crate::models::player::PlayerEntity;
use crate::models::response::ApiResponse;
use crate::services::db::Db;

pub async fn get_random_players(State(state): State<AppState>) -> Result<ApiResponse<Vec<PlayerEntity>>, ApiError> {
    let players = Db::get_players(&state.db_connection)
        .await?;

    Ok(ApiResponse::JsonData(players))
}
