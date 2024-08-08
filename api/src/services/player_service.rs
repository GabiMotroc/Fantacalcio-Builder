use crate::models::app_state::AppState;
use crate::models::error::ApiError;
use crate::models::player::PlayerEntity;
use crate::models::response::ApiResponse;
use crate::models::user::User;
use crate::services::db::Db;
use axum::extract::State;
use axum::Json;

pub async fn get_players(State(state): State<AppState>) -> Result<ApiResponse<Vec<PlayerEntity>>, ApiError> {
    let players = Db::get_players(&state.db_connection)
        .await?;

    Ok(ApiResponse::JsonData(players))
}

pub async fn save_selected_players(user: User, State(state): State<AppState>, Json(player_ids): Json<Vec<i32>>) -> Result<ApiResponse<bool>, ApiError> {
    Db::select_players(&state.db_connection, user.id, player_ids).await?;

    Ok(ApiResponse::Ok)
}

pub async fn get_selected_players(user: User, State(state): State<AppState>) -> Result<ApiResponse<Vec<PlayerEntity>>, ApiError> {
    let players = Db::get_selected_players(&state.db_connection, user.id)
        .await?;

    Ok(ApiResponse::JsonData(players))
}
