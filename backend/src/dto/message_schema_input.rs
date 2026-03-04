use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::models::message::{MessageLocation, MessageType};

// requête FRONTEND
#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct SimpleMessageSchema {
    pub content: String,
}

// requête INTERNE (backend)
#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct SendMessageSchema {
    pub content: String,
    pub message_type: MessageType,
    pub location: MessageLocation,
    pub location_id: String,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct UpdateMessageSchema {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_type: Option<MessageType>,
}