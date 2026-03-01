use axum::{Json, response::IntoResponse};
use rand::RngExt;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    ApiError,
    utils::{count_all_models, get_base_url},
};

const MEDIA_FOLDER: &str = "dumpster";

#[derive(Debug, Serialize, Deserialize)]
struct Attribution {
    pub attribution: String,
}

pub async fn get_random_raccoon_model() -> Result<impl IntoResponse, ApiError> {
    let models_count = count_all_models()
        .await
        .expect("Failed to count all 3d models");

    let random_id = rand::rng().random_range(1..=models_count);

    let model_url = format!(
        "{}/{}/models3d/racc{}.glb",
        get_base_url(),
        MEDIA_FOLDER,
        random_id
    );

    let models_3d_attribution =
        tokio::fs::read_to_string(format!("{}/models3d/racc{}.json", MEDIA_FOLDER, random_id))
            .await
            .map_err(|_| ApiError::NotFound)?;

    let attribution_data: Attribution =
        serde_json::from_str(&models_3d_attribution).map_err(|_| ApiError::NotFound)?;

    Ok(Json(json!({
        "status": "ok",
        "model": model_url,
        "attribution": attribution_data.attribution
    })))
}
