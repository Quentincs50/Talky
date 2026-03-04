pub mod handlers;
pub mod error;

use axum::{
    routing::{get, post, patch, delete},
    Router,
};
use utoipa::OpenApi;

use handlers::{
    // GET
    get_channel_messages_handler,
    get_private_messages_handler,
    get_server_messages_handler,

    // POST
    send_channel_message_handler,
    send_private_message_handler,
    send_server_message_handler,
    create_message_handler,

    // UPDATE
    edit_message_handler,

    // DELETE
    delete_message_handler,
};
#[derive(OpenApi)]
#[openapi(
    paths(
        // GET
        handlers::get_channel_messages_handler,
        handlers::get_private_messages_handler,
        handlers::get_server_messages_handler,


        // POST
        handlers::send_channel_message_handler,
        handlers::send_private_message_handler,
        handlers::send_server_message_handler,
        handlers::create_message_handler,

        // UPDATE
        handlers::edit_message_handler,

        // DELETE
        handlers::delete_message_handler,
    ),
    components(
        schemas(
            crate::dto::message_schema_input::SimpleMessageSchema,
            crate::dto::message_schema_input::SendMessageSchema,
            crate::dto::message_schema_input::UpdateMessageSchema,
            crate::dto::message_schema_response::MessageResponse,
            crate::dto::message_schema_response::MessageListResponse,
            crate::dto::message_schema_response::SingleMessageResponse,
            crate::models::message::MessageModel,
        )
    ),
    tags(
        (name = "messages", description = "Gestion des messages")
    )
)]
pub struct ApiDoc;

pub fn router() -> Router {
    Router::new()
        // -------- GET --------
        .route(
            "/api/channels/{channel_id}/messages",
            get(get_channel_messages_handler),
        )
        .route(
            "/api/private/{user_id}/messages",
            get(get_private_messages_handler),
        )
        .route(
            "/api/servers/{server_id}/messages",
            get(get_server_messages_handler),
        )
        
        // -------- POST --------
        .route(
            "/api/channels/{channel_id}/messages",
            post(send_channel_message_handler),
        )
        .route(
            "/api/private/{user_id}/messages",
            post(send_private_message_handler),
        )
        .route(
            "/api/servers/{server_id}/messages",
            post(send_server_message_handler),
        )
        .route(
            "/",
            post(create_message_handler),
        )

        // -------- UPDATE --------
        .route(
            "/api/messages/{id}",
            patch(edit_message_handler),
        )

        // -------- DELETE --------
        .route(
            "/api/messages/{id}",
            delete(delete_message_handler),
        )
}
