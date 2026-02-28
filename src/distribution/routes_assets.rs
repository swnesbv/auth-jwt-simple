use axum::{Router};
use tower_http::services::{ServeDir};
use std::{path::PathBuf};


pub fn build_rt() -> Router {

    let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("static");
    Router::new()
        .fallback_service(
            ServeDir::new(assets_dir).append_index_html_on_directories(true)
        )
}
