use axum::{Router, routing::{get, patch, post}};
use axum::http::{HeaderName, Method};
use tower_http::cors::{Any, CorsLayer};

use crate::handlers::upload::receive_chunks;

pub fn create_router() -> Router {
    let x_file_name = HeaderName::from_static("x-file-name");
    let upload_offset = HeaderName::from_static("upload-offset");

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::POST, Method::OPTIONS, Method::GET, Method::PATCH])
        .allow_headers([
            x_file_name,
            upload_offset,
            axum::http::header::CONTENT_TYPE,
        ]);

    Router::new()
        .route("/upload", patch(receive_chunks))
        .route("/", get(||async{"hello, world"}))
        .layer(cors)
}
