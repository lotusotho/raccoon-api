use axum::{
    Json, Router,
    response::{IntoResponse, Redirect, Result},
    routing::get,
};
use dotenvy::var;
use std::sync::{Arc, atomic::AtomicU16};

use crate::{
    facts::get_random_fact,
    games::get_games,
    memes::{get_meme, get_random_meme},
    models3d::get_random_raccoon_model,
    raccoons::{get_raccoon, get_random_raccoon, raccoon_of_the_day},
    sounds::get_random_sound,
    state::AppState,
    utils::{count_all_media, fetch_status_code, raccoon_of_the_day_scheduler},
    videos::{get_random_video, get_video},
    wiki::get_wiki_redirect,
};
use serde_json::{Value, json};
use tower::Layer;
use tower_governor::{GovernorLayer, governor::GovernorConfigBuilder};
use tower_http::{normalize_path::NormalizePathLayer, services::ServeDir};

mod facts;
mod games;
mod memes;
mod models3d;
mod raccoons;
mod sounds;
mod state;
mod utils;
mod videos;
mod wiki;

#[derive(Debug)]
enum ApiError {
    NotFound,
    InternalError,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        match self {
            ApiError::NotFound => fetch_status_code(404),
            ApiError::InternalError => fetch_status_code(500),
        }
    }
}

async fn health_check() -> impl IntoResponse {
    Json(json!({
        "status": "ok",
        "message": "Server is running",
    }))
}

async fn get_root() -> Result<Json<Value>, ApiError> {
    Ok(Json(json!({
        "status": "ok",
        "message": "The raccoon api! 🦝",
        "attribution": "Thanks to api.racc.lol repo and Venqoi for the vast mayority of assets! 🦝 https://github.com/raccoonOrg/api.racc.lol"
    })))
}

async fn redirect_to_root() -> Result<impl IntoResponse, ApiError> {
    Ok(Redirect::to("/v1"))
}

fn create_app(state: AppState) -> Router {
    let governor_conf = Arc::new(
        GovernorConfigBuilder::default()
            .per_second(10)
            .burst_size(100)
            .finish()
            .unwrap(),
    );

    let version = var("VERSION").unwrap_or_else(|_| format!("v1"));

    let api_routes = Router::new()
        .route("/", get(get_root))
        .route("/health", get(health_check))
        .route("/thiscoon/{id}", get(get_raccoon))
        .route("/coon", get(get_random_raccoon))
        .route("/meme", get(get_random_meme))
        .route("/thismeme/{id}", get(get_meme))
        .route("/vid", get(get_random_video))
        .route("/thisvid/{id}", get(get_video))
        .route("/rotd", get(raccoon_of_the_day))
        .route("/fact/{locale}", get(get_random_fact))
        .route("/model", get(get_random_raccoon_model))
        .route("/sound", get(get_random_sound))
        .route("/wiki", get(get_wiki_redirect))
        .route("/games", get(get_games))
        .with_state(state.clone());

    Router::new()
        .nest(&format!("/{}", version), api_routes)
        .route("/", get(redirect_to_root))
        .layer(GovernorLayer::new(governor_conf))
        .with_state(state)
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let raccoon_images_count = count_all_media("raccs")
        .await
        .expect("Failed to count all raccoon images");
    let state = AppState {
        raccoon_images_count,
        meme_images_count: count_all_media("memes")
            .await
            .expect("Failed to count all meme images"),
        video_count: count_all_media("videos")
            .await
            .expect("Failed to count all videos"),
        raccoon_of_the_day: Arc::new(AtomicU16::new(rand::random_range(1..=raccoon_images_count))),
    };

    let normalized_service =
        NormalizePathLayer::trim_trailing_slash().layer(create_app(state.clone()));

    let app = Router::new().fallback_service(normalized_service);

    let host = var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = var("PORT").unwrap_or_else(|_| "3000".to_string());

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", host, port))
        .await
        .expect("failed to bind tcp listener");

    println!("Server running on  {}:{}", host, port);

    tokio::spawn(async move { raccoon_of_the_day_scheduler(state).await });

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .await
    .expect("failed to start server");
}
