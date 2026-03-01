use axum::{
    body::Body,
    extract::{Path, State},
    http::{Response, header},
    response::IntoResponse,
};
use rand::RngExt;

use crate::{ApiError, state::AppState};

const MEDIA_FOLDER: &str = "dumpster";

pub async fn get_video(Path(id): Path<u32>) -> Result<impl IntoResponse, ApiError> {
    let image_content = tokio::fs::read(format!("{}/videos/racc{}.mp4", MEDIA_FOLDER, id))
        .await
        .map_err(|_| ApiError::NotFound)?;

    let response = Response::builder()
        .header(header::CONTENT_TYPE, "video/mp4")
        .header(header::CONTENT_LENGTH, image_content.len())
        .body(Body::from(image_content))
        .map_err(|_| ApiError::InternalError)?;

    Ok(response)
}

pub async fn get_random_video(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, ApiError> {
    let video_count = state.video_count;

    if video_count == 0 {
        return Err(ApiError::NotFound);
    }

    let random_id = rand::rng().random_range(1..=video_count);

    let image_content = tokio::fs::read(format!("{}/videos/racc{}.mp4", MEDIA_FOLDER, random_id))
        .await
        .map_err(|_| ApiError::NotFound)?;

    let response = Response::builder()
        .header(header::CONTENT_TYPE, "video/mp4")
        .header(header::CONTENT_LENGTH, image_content.len())
        .body(Body::from(image_content))
        .map_err(|_| ApiError::InternalError)?;

    Ok(response)
}
