use axum::{
    body::Body,
    extract::{Path, State},
    http::{Response, header},
    response::IntoResponse,
};
use rand::RngExt;

use crate::{ApiError, AppState};

const MEDIA_FOLDER: &str = "dumpster";

pub async fn get_meme(Path(id): Path<u32>) -> Result<impl IntoResponse, ApiError> {
    let image_content = tokio::fs::read(format!("{}/memes/racc{}.jpg", MEDIA_FOLDER, id))
        .await
        .map_err(|_| ApiError::NotFound)?;

    let response = Response::builder()
        .header(header::CONTENT_TYPE, "image/jpeg")
        .header(header::CONTENT_LENGTH, image_content.len())
        .body(Body::from(image_content))
        .map_err(|_| ApiError::InternalError)?;

    Ok(response)
}

pub async fn get_random_meme(State(state): State<AppState>) -> Result<impl IntoResponse, ApiError> {
    let images_count = state.meme_images_count;

    if images_count == 0 {
        return Err(ApiError::NotFound);
    }

    let random_id = rand::rng().random_range(1..=images_count);

    let image_content = tokio::fs::read(format!("{}/memes/racc{}.jpg", MEDIA_FOLDER, random_id))
        .await
        .map_err(|_| ApiError::NotFound)?;

    let response = Response::builder()
        .header(header::CONTENT_TYPE, "image/jpeg")
        .header(header::CONTENT_LENGTH, image_content.len())
        .body(Body::from(image_content))
        .map_err(|_| ApiError::InternalError)?;

    Ok(response)
}
