use axum::response::{IntoResponse, Redirect};

use crate::ApiError;

pub async fn get_wiki_redirect() -> Result<impl IntoResponse, ApiError> {
    Ok(Redirect::to("https://en.wikipedia.org/wiki/Raccoon"))
}
