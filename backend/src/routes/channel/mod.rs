use axum::{Router, routing::{put, get, delete}};
use utoipa::OpenApi;

pub mod channel;

#[derive(OpenApi)]
#[openapi(
    paths(
        channel::get_channel,
        channel::update_channel,
        channel::delete_channel
    )
)]
pub struct ApiDoc;

pub fn router() -> Router {
    Router::new()
        .route("/channel/{channel_id}", get(channel::get_channel))
        .route("/channel/{channel_id}", put(channel::update_channel))
        .route("/channel/{channel_id}", delete(channel::delete_channel))
        
}

