use axum::{
    body::Body,
    extract::{Path, State},
    http::{Response, header},
    response::{IntoResponse, Redirect},
};
use rand::RngExt;

use crate::{ApiError, AppState};

const MEDIA_FOLDER: &str = "dumpster";

pub async fn get_raccoon(Path(id): Path<u32>) -> Result<impl IntoResponse, ApiError> {
    let image_content = tokio::fs::read(format!("{}/raccs/racc{}.jpg", MEDIA_FOLDER, id))
        .await
        .map_err(|_| ApiError::NotFound)?;

    let response = Response::builder()
        .header(header::CONTENT_TYPE, "image/jpeg")
        .header(header::CONTENT_LENGTH, image_content.len())
        .body(Body::from(image_content))
        .map_err(|_| ApiError::InternalError)?;

    Ok(response)
}

pub async fn get_random_raccoon(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, ApiError> {
    let images_count = state.raccoon_images_count;

    if images_count == 0 {
        return Err(ApiError::NotFound);
    }

    let random_id = rand::rng().random_range(1..=images_count);

    let image_content = tokio::fs::read(format!("{}/raccs/racc{}.jpg", MEDIA_FOLDER, random_id))
        .await
        .map_err(|_| ApiError::NotFound)?;

    let response = Response::builder()
        .header(header::CONTENT_TYPE, "image/jpeg")
        .header(header::CONTENT_LENGTH, image_content.len())
        .body(Body::from(image_content))
        .map_err(|_| ApiError::InternalError)?;

    Ok(response)
}

pub async fn raccoon_of_the_day(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, ApiError> {
    let winner = state
        .raccoon_of_the_day
        .load(std::sync::atomic::Ordering::Relaxed);

    Ok(Redirect::to(&format!("/thiscoon/{}", winner)))
}
