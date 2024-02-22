use axum::{Router, routing::get};
use tower_http::services::ServeDir;
use std::path::Path;
use super::handlers;

pub fn router(assets_path: &Path) -> Router {
    Router::new()
        .route("/", get(handlers::root))
        .route("/blog", get(handlers::blog))
        .route("/blog/post", get(handlers::blog_post))
        .route("/resume", get(handlers::resume))
        .route("/projects", get(handlers::projects))
        .nest_service("/assets", ServeDir::new(assets_path))
}
