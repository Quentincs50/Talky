pub mod handlers;

use axum::{Router, routing::get};

pub fn router() -> Router {
    Router::new()
        .route("/ws", get(handlers::ws_handler))
}