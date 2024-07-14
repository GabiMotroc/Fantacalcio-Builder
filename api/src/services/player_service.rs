use axum::extract::State;
use axum::Json;

use crate::models::app_state::AppState;
use crate::models::error::ApiError;
use crate::models::player::PlayerEntity;
use crate::models::response::ApiResponse;
use crate::models::user::User;
use crate::services::db::Db;

pub async fn get_random_players(user: User, State(state): State<AppState>) -> Result<ApiResponse<Vec<PlayerEntity>>, ApiError> {
    let players = Db::get_players(&state.db_connection)
        .await?;

    Ok(ApiResponse::JsonData(players))
}

pub async fn save_selected_players(user: User, State(state): State<AppState>, Json(player_ids): Json<Vec<i32>>) -> Result<ApiResponse<bool>, ApiError> {
    Ok(ApiResponse::Ok)
}