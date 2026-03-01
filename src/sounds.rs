use axum::{
    body::Body,
    http::{Response, header},
    response::IntoResponse,
};
use rand::RngExt;

use crate::{ApiError, utils::count_all_media};

const MEDIA_FOLDER: &str = "dumpster";

pub async fn get_random_sound() -> Result<impl IntoResponse, ApiError> {
    let sound_count = count_all_media("sounds")
        .await
        .expect("Failed to count all 3d models");

    let random_id = rand::rng().random_range(1..=sound_count);

    let sound_content = tokio::fs::read(format!("{}/sounds/racc{}.mp3", MEDIA_FOLDER, random_id))
        .await
        .map_err(|_| ApiError::NotFound)?;

    let response = Response::builder()
        .header(header::CONTENT_TYPE, "audio/mpeg")
        .body(Body::from(sound_content))
        .map_err(|_| ApiError::InternalError)?;

    Ok(response)
}
