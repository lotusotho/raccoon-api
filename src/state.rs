use std::sync::{Arc, atomic::AtomicU16};

#[derive(Clone)]
pub struct AppState {
    pub raccoon_images_count: u16,
    pub meme_images_count: u16,
    pub video_count: u16,
    pub raccoon_of_the_day: Arc<AtomicU16>,
}
