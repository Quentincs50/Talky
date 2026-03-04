
use axum::{
    extract::{Path, Extension},
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::services::message_service::MessageService;
use crate::routes::auth::auth::AuthUser;
use crate::dto::message_schema_input::{SimpleMessageSchema, SendMessageSchema, UpdateMessageSchema};
use crate::dto::message_schema_response::{MessageResponse, MessageListResponse, SingleMessageResponse};
use crate::models::message::*;
use super::error::{MyError};
use tokio::sync::broadcast;
use mongodb::Database;


#[utoipa::path(
    get,
    path = "/channels/{channel_id}/messages",
    params(
        ("channel_id" = String, Path, description = "Channel ID")
    ),
    responses(
        (status = 200, description = "Messages retrieved", body = MessageListResponse),
    ),
    tag = "messages",
    security(("bearer_auth" = []))
)]

pub async fn get_channel_messages_handler(
    AuthUser(user): AuthUser,
    Path(channel_id): Path<String>,
    Extension(db): Extension<Database>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let message_service = MessageService::new(&db);
    
    match message_service.get_messages(MessageLocation::Channel, channel_id).await {
          Ok(response) => Ok(Json(response)),
          Err(_) => Err(MyError::NotFoundError("Messages not found".to_string()).into()),
      }
}

#[utoipa::path(
    get,
    path = "/private/{user_id}/messages",
    params(
        ("user_id" = String, Path, description = "Other user ID")
    ),
    responses(
        (status = 200, description = "Private messages retrieved", body = MessageListResponse),
    ),
    tag = "messages",
    security(("bearer_auth" = []))
)]
pub async fn get_private_messages_handler(
      AuthUser(user): AuthUser,
      Path(other_user_id): Path<String>,
      Extension(db): Extension<Database>,
  ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
      let message_service = MessageService::new(&db);


      match message_service.get_private_messages(user.sub, other_user_id).await {
          Ok(response) => Ok(Json(response)),
          Err(_) => Err(MyError::NotFoundError("Messages not found".to_string()).into()),
      }
  }



#[utoipa::path(
    get,
    path = "/servers/{server_id}/messages",
    params(
        ("server_id" = String, Path, description = "Server ID")
    ),
    responses(
        (status = 200, description = "Server messages retrieved", body = MessageListResponse),
    ),
    tag = "messages",
    security(("bearer_auth" = []))
)]
pub async fn get_server_messages_handler(
    AuthUser(user): AuthUser,
    Path(server_id): Path<String>,
    Extension(db): Extension<Database>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let message_service = MessageService::new(&db);
    
    match message_service.get_messages(MessageLocation::Server, server_id).await {
          Ok(response) => Ok(Json(response)),
          Err(_) => Err(MyError::NotFoundError("Messages not found".to_string()).into()),
      }
}


#[utoipa::path(
    post,
    path = "/channels/{channel_id}/messages",
    params(
        ("channel_id" = String, Path, description = "Channel ID")
    ),
    request_body = SimpleMessageSchema,
    responses(
        (status = 201, description = "Message sent to channel", body = SingleMessageResponse),
    ),
    tag = "messages",
    security(("bearer_auth" = []))
)]

pub async fn send_channel_message_handler(
    AuthUser(user): AuthUser,
    Path(channel_id): Path<String>,
    Extension(db): Extension<Database>,
    Extension(tx): Extension<broadcast::Sender<String>>,
    Json(body): Json<SimpleMessageSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let message_service = MessageService::new(&db);

    let full_body = SendMessageSchema {
        content: body.content,
        message_type: MessageType::Text, 
        location: MessageLocation::Channel, 
        location_id: channel_id, 
    };

    match message_service.create_message(user.sub, full_body).await {
          Ok(res) => {
            let json = serde_json::to_string(&res).unwrap_or_default();
            let _ = tx.send(json);
            Ok((StatusCode::CREATED, Json(res)))
        },
          Err(e) => Err(e.into()),
      }
}


#[utoipa::path(
    post,
    path = "/private/{user_id}/messages",
    params(
        ("user_id" = String, Path, description = "Recipient user ID")
    ),
    request_body = SimpleMessageSchema,
    responses(
        (status = 201, description = "Private message sent", body = SingleMessageResponse),
    ),
    tag = "messages",
    security(("bearer_auth" = []))
)]
pub async fn send_private_message_handler(
    AuthUser(user): AuthUser,
    Path(user_id): Path<String>,
    Extension(db): Extension<Database>,
    Extension(tx): Extension<broadcast::Sender<String>>,
    Json(body): Json<SimpleMessageSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let message_service = MessageService::new(&db);


    let full_body = SendMessageSchema {
        content: body.content,
        message_type: MessageType::Text,
        location: MessageLocation::Private, 
        location_id: user_id, 
    };

    match message_service.create_message(user.sub, full_body).await {
          Ok(res) => {
            let json = serde_json::to_string(&res).unwrap_or_default();
            let _ = tx.send(json);
            Ok((StatusCode::CREATED, Json(res)))
        },
          Err(e) => Err(e.into()),
      }
}

#[utoipa::path(
    post,
    path = "/servers/{server_id}/messages",
    params(
        ("server_id" = String, Path, description = "Server ID")
    ),
    request_body = SimpleMessageSchema,
    responses(
        (status = 201, description = "Message sent to server", body = SingleMessageResponse),
    ),
    tag = "messages",
    security(("bearer_auth" = []))
)]
pub async fn send_server_message_handler(
    AuthUser(user): AuthUser,
    Path(server_id): Path<String>,
    Extension(db): Extension<Database>,
    Extension(tx): Extension<broadcast::Sender<String>>,
    Json(body): Json<SimpleMessageSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let message_service = MessageService::new(&db);


    let full_body = SendMessageSchema {
        content: body.content,
        message_type: MessageType::Text,
        location: MessageLocation::Server, 
        location_id: server_id,  
    };

    match message_service.create_message(user.sub, full_body).await {
        Ok(res) => {
            let json = serde_json::to_string(&res).unwrap_or_default();
            let _ = tx.send(json);
            Ok((StatusCode::CREATED, Json(res)))
        },
          Err(e) => Err(e.into()),
    }
}

#[utoipa::path(
    post,
    path = "/messages",
    request_body = SendMessageSchema,
    responses(
        (status = 201, description = "Message created successfully", body = SingleMessageResponse),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error")
    ),
    tag = "messages",
    security(("bearer_auth" = []))
)]
pub async fn create_message_handler(
    AuthUser(user): AuthUser,
    Extension(db): Extension<Database>,
    Extension(tx): Extension<broadcast::Sender<String>>,
    Json(body): Json<SendMessageSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    let message_service = MessageService::new(&db);

    match message_service.create_message(user.sub, body).await.map_err(MyError::from) {
          Ok(res) => {
            let json = serde_json::to_string(&res).unwrap_or_default();
            let _ = tx.send(json);
            Ok((StatusCode::CREATED, Json(res)))
        },
          Err(e) => Err(e.into()),
      }
}


#[utoipa::path(
    patch,
    path = "/messages/{id}",
    params(
        ("id" = String, Path, description = "Message ID")
    ),
    request_body = UpdateMessageSchema,
    responses(
        (status = 200, description = "Message updated", body = MessageModel),
        (status = 404, description = "Message not found")
    ),
    tag = "messages",
    security(("bearer_auth" = []))
)]
pub async fn edit_message_handler(
    AuthUser(user): AuthUser,
    Path(id): Path<String>,
    Extension(db): Extension<Database>,
    Json(body): Json<UpdateMessageSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let message_service = MessageService::new(&db);

    match message_service.edit_message(&id, body).await {
        Ok(res) => Ok(Json(res)),
        Err(_) => Err(MyError::NotFoundError(id).into()),
    }
}

#[utoipa::path(
    delete,
    path = "/messages/{id}",
    params(
        ("id" = String, Path, description = "Message ID")
    ),
    responses(
        (status = 204, description = "Message deleted"),
        (status = 404, description = "Message not found")
    ),
    tag = "messages",
    security(("bearer_auth" = []))
)]
pub async fn delete_message_handler(
    AuthUser(user): AuthUser,
    Path(id): Path<String>,
    Extension(db): Extension<Database>,
    Extension(tx): Extension<broadcast::Sender<String>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let message_service = MessageService::new(&db);

    match message_service.delete_message(&id).await {
        Ok(res) => {
            let json = serde_json::to_string(&res).unwrap_or_default();
            let _ = tx.send(json);
            Ok((StatusCode::NO_CONTENT, Json(res)))
        },
          Err(_) => Err(MyError::NotFoundError(id).into()),
    }
}
