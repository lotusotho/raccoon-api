use axum::{Json, response::IntoResponse};
use serde::{Deserialize, Serialize};

use crate::ApiError;

#[derive(Deserialize, Serialize)]
struct GameResponse {
    #[serde(flatten)]
    details: std::collections::HashMap<String, GameData>,
}

#[derive(Deserialize, Serialize)]
struct GameData {
    data: ReleaseInfo,
}

#[derive(Deserialize, Serialize)]
struct ReleaseInfo {
    name: String,
    release_date: ComingSoon,
}

#[derive(Deserialize, Serialize)]
struct ComingSoon {
    coming_soon: bool,
}

#[derive(Serialize)]
struct ResponseData {
    game_name: String,
    has_released: bool,
    status: String,
}

const GAMES_IDS: [&str; 3] = ["3599690", "3376710", "2700430"];

pub async fn get_games() -> Result<impl IntoResponse, ApiError> {
    let mut games_from_steam: Vec<GameResponse> = Vec::new();

    for game_id in GAMES_IDS {
        let get_game_from_steam: serde_json::Value = reqwest::get(&format!(
            "https://store.steampowered.com/api/appdetails?appids={}",
            game_id
        ))
        .await
        .map_err(|_| ApiError::InternalError)?
        .json()
        .await
        .map_err(|_| ApiError::InternalError)?;

        let game_data: GameResponse =
            serde_json::from_value(get_game_from_steam).map_err(|_| ApiError::NotFound)?;

        games_from_steam.push(game_data);
    }

    let converted_data: Vec<ResponseData> = games_from_steam
        .into_iter()
        .flat_map(|game| {
            game.details
                .into_iter()
                .map(|(_id, game_data)| ResponseData {
                    status: "ok".to_string(),
                    game_name: game_data.data.name,
                    has_released: !game_data.data.release_date.coming_soon,
                })
        })
        .collect();

    Ok(Json(converted_data))
}
