use axum::{Router, routing::{post, get}};
use utoipa::OpenApi;

pub mod invitations;

#[derive(OpenApi)]
#[openapi(
    paths(
        invitations::create_invitation,
        invitations::join_with_invite
    )
)]
pub struct ApiDoc;

pub fn router() -> Router {
    Router::new()
        .route("/invitations/{server_id}/invitations", post(invitations::create_invitation))
        .route("/invitations/{code}/join", post(invitations::join_with_invite))
}
