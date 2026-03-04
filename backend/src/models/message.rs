use chrono::prelude::*;
use mongodb::bson::{self, oid::ObjectId};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum MessageLocation {
    Channel,
    Private,
    Server,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum MessageType {
    Text,
    Image,
    System,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct MessageModel {
      #[serde(rename = "_id")]
      #[schema(value_type = String)]
      pub id: ObjectId,
      pub owner: String,
      pub content: String,
      pub message_type: MessageType,
      pub location_id: String,          
      pub location: MessageLocation,
      pub edited: Option<bool>,
      #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
      #[schema(value_type = String, format = "date-time")]
      pub createdAt: DateTime<Utc>,
      #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
      #[schema(value_type = String, format = "date-time")]
      pub updatedAt: DateTime<Utc>,
  }