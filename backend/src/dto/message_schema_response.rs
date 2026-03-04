use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;
use crate::models::message::{MessageLocation, MessageType};

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug, ToSchema)]
pub struct MessageResponse {
    pub id: String,
    pub owner: String,
    pub content: String,
    pub message_type: MessageType,
    pub location: MessageLocation,
    pub location_id: String, 
    pub edited: Option<bool>,
    #[schema(value_type = String, format = "date-time")]
    pub createdAt: DateTime<Utc>,
    #[schema(value_type = String, format = "date-time")]
    pub updatedAt: DateTime<Utc>,
}

#[derive(Serialize, Debug, ToSchema)]
pub struct SingleMessageResponse {
    pub status: &'static str,
    pub message: MessageResponse,
}

#[derive(Serialize, Debug, ToSchema)]
pub struct MessageListResponse {
    pub status: &'static str,
    pub results: usize,
    pub messages: Vec<MessageResponse>,
}