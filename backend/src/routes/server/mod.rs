use axum::{
    Router,
    routing::{get, post, delete, patch},
};

use utoipa::OpenApi;

pub mod server;
use crate::routes::channel::channel;
#[derive(OpenApi)]
#[openapi(
    paths(
        server::create_server,
        server::list_servers,
        server::join_server,
        server::leave_server,
        server::list_server_members,
        server::update_member_role,
        server::delete_server,
        server::transfer_ownership,
        channel::create_channel,
        channel::list_channels,
    ),
    tags(
        (name = "server", description = "Server management")
    )
)]
pub struct ApiDoc;

pub fn router() -> Router {
    Router::new()
        .route("/server/createServer", post(server::create_server))
        .route("/server/listServers", get(server::list_servers))
        .route("/server/{server_id}/join", post(server::join_server))
        .route("/server/{server_id}/leave", delete(server::leave_server))
        .route("/server/{server_id}/members", get(server::list_server_members))
        .route("/server/{server_id}/members/{id_user}/role",patch(server::update_member_role))
        .route("/server/deleteServer/{server_id}", delete(server::delete_server))
        .route("/server/{server_id}/transfer/{new_owner_id}",post(server::transfer_ownership))
        .route("/server/{server_id}/channels", post(channel::create_channel))
        .route("/server/{server_id}/channels", get(channel::list_channels))
}
