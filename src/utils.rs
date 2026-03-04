use std::io;

use axum::{
    body::Body,
    http::{Response, StatusCode, header},
    response::IntoResponse,
};
use tokio::time::{Duration, Instant, sleep_until};
use tokio_cron_scheduler::{Job, JobScheduler};

use crate::state::AppState;

const MEDIA_FOLDER: &str = "dumpster";

pub async fn count_all_media(path: &str) -> io::Result<u16> {
    let mut images_count = 0;
    let mut entries = tokio::fs::read_dir(&format!("{}/{}", MEDIA_FOLDER, path)).await?;

    while let Some(entry) = entries.next_entry().await? {
        let file_type = entry.file_type().await?;

        if file_type.is_file() && entry.file_name() != ".gitkeep" {
            images_count += 1;
        }
    }

    Ok(images_count)
}

pub async fn count_all_models() -> io::Result<u16> {
    let mut models_count = 0;
    let mut entries = tokio::fs::read_dir(&format!("{}/models3d", MEDIA_FOLDER)).await?;

    while let Some(entry) = entries.next_entry().await? {
        let file_type = entry.file_type().await?;
        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy();

        if file_type.is_file() && file_name_str.ends_with(".glb") && file_name != ".gitkeep" {
            models_count += 1;
        }
    }

    Ok(models_count)
}

pub fn fetch_status_code(code: u16) -> axum::response::Response {
    match std::fs::read(format!("{}/status_codes/{}.png", MEDIA_FOLDER, code)) {
        Ok(image_content) => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .header(header::CONTENT_TYPE, "image/jpeg")
            .body(Body::from(image_content))
            .unwrap()
            .into_response(),
        Err(_) => (StatusCode::NOT_FOUND, "Raccoon not found").into_response(),
    }
}

pub async fn raccoon_of_the_day_scheduler(state: AppState) {
    let scheduler = JobScheduler::new().await.unwrap();

    // Runs everyday
    scheduler
        .add(
            Job::new_async("0 0 0 * * *", move |_uuid, _locked| {
                let state = state.clone();

                Box::pin(async move {
                    println!("New ROTD executed");

                    state.raccoon_of_the_day.store(
                        rand::random_range(1..=state.raccoon_images_count),
                        std::sync::atomic::Ordering::Relaxed,
                    );
                })
            })
            .unwrap(),
        )
        .await
        .unwrap();

    scheduler.start().await.unwrap();

    // Keep the scheduler running
    loop {
        sleep_until(Instant::now() + Duration::from_secs(60)).await;
    }
}
