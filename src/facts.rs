use axum::{Json, extract::Path, response::IntoResponse};
use rand::seq::IteratorRandom;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::ApiError;

const MEDIA_FOLDER: &str = "./dumpster";

#[derive(Debug, Serialize, Deserialize)]
struct Facts {
    pub facts: Vec<String>,
}

pub async fn get_random_fact_default() -> Result<impl IntoResponse, ApiError> {
    get_random_fact(Path("en".to_string())).await
}

pub async fn get_random_fact(Path(locale): Path<String>) -> Result<impl IntoResponse, ApiError> {
    if locale != "en".to_string() && locale != "es".to_string() {
        Err(ApiError::NotFound)?;
    }

    let facts_content =
        tokio::fs::read_to_string(format!("{}/facts/facts_{}.json", MEDIA_FOLDER, locale))
            .await
            .map_err(|_| ApiError::NotFound)?;

    let facts_data: Facts = serde_json::from_str(&facts_content).map_err(|_| ApiError::NotFound)?;

    let random_fact = facts_data
        .facts
        .into_iter()
        .choose(&mut rand::rng())
        .ok_or(ApiError::NotFound)?;

    Ok(Json(json!({
        "status": "ok",
        "fact": random_fact
    })))
}
