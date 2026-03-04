use crate::dto::message_schema_input::{SendMessageSchema, UpdateMessageSchema};
use crate::dto::message_schema_response::{
    MessageListResponse, MessageResponse, SingleMessageResponse,
};
use crate::models::message::{MessageLocation, MessageModel, MessageType};
use crate::routes::message::error::MyError;
use crate::routes::message::error::MyError::*;
use futures::StreamExt;
use mongodb::bson::{doc, oid::ObjectId};
use mongodb::{Collection, Database};
pub struct MessageService {
    collection: Collection<MessageModel>,
}

impl MessageService {
    pub fn new(db: &Database) -> Self {
        let collection_name = std::env::var("MONGODB_MESSAGE_COLLECTION")
            .expect("MONGODB_MESSAGE_COLLECTION must be set");

        Self {
            collection: db.collection(&collection_name.as_str()),
        }
    }

    pub async fn create_message(
        &self,
        owner: String,
        body: SendMessageSchema,
    ) -> Result<SingleMessageResponse, MyError> {
        
    let message = MessageModel {
      id: ObjectId::new(),
      owner,
      content: body.content,
      message_type: body.message_type,
      location: body.location,
      location_id: body.location_id,
      edited: Some(false),
      createdAt: chrono::Utc::now(),
      updatedAt: chrono::Utc::now(),
  };

        let insert_result = match self.collection.insert_one(&message).await {
            Ok(result) => result,
            Err(e) => {
                if e.to_string()
                    .contains("E11000 duplicate key error collection")
                {
                    return Err(MongoDuplicateError(e));
                }
                return Err(MongoQueryError(e));
            }
        };

        let new_id = insert_result
            .inserted_id
            .as_object_id()
            .expect("issue with new _id");

        let message_doc = match self.collection.find_one(doc! {"_id": new_id}).await {
            Ok(Some(doc)) => doc,
            Ok(None) => return Err(NotFoundError(new_id.to_string())),
            Err(e) => return Err(MongoQueryError(e)),
        };

        Ok(SingleMessageResponse {
            status: "success",
            message: self.doc_to_message(&message_doc),
        })
    }


    pub async fn get_messages(
        &self,
        location: MessageLocation,
        location_id: String,
    ) -> Result<MessageListResponse, MyError> {
        let filter = doc! {
            "location": bson::to_bson(&location).map_err(MongoSerializeBsonError)?,
            "location_id": &location_id
        };

        let mut cursor = self
            .collection
            .find(filter)
            .sort(doc! {"createdAt": 1})
            .await
            .map_err(MongoQueryError)?;

        let mut messages: Vec<MessageResponse> = Vec::new();
        while let Some(doc) = cursor.next().await {
            messages.push(self.doc_to_message(&doc.map_err(MongoQueryError)?));
        }

        Ok(MessageListResponse {
            status: "success",
            results: messages.len(),
            messages,
        })
    }

    pub async fn get_private_messages(
        &self,
        user1_id: String,
        user2_id: String,
    ) -> Result<MessageListResponse, MyError> {
        let filter = doc! {
            "location": bson::to_bson(&MessageLocation::Private).map_err(MongoSerializeBsonError)?,
            "$or": [
                {"owner": &user1_id, "location_id": &user2_id},
                {"owner": &user2_id, "location_id": &user1_id}
            ]
        };

        let mut cursor = self
            .collection
            .find(filter)
            .sort(doc! {"createdAt": 1})
            .await
            .map_err(MongoQueryError)?;

        let mut messages: Vec<MessageResponse> = Vec::new();
        while let Some(doc) = cursor.next().await {
            messages.push(self.doc_to_message(&doc.map_err(MongoQueryError)?));
        }

        Ok(MessageListResponse {
            status: "success",
            results: messages.len(),
            messages,
        })
    }

    pub async fn get_private_message(&self, id: &str) -> Result<SingleMessageResponse, MyError> {
        let oid = ObjectId::parse_str(id).map_err(|_| InvalidIDError(id.to_owned()))?;

        let message_doc = self.collection
        .find_one(doc! {
            "_id": oid,
            "location": bson::to_bson(&MessageLocation::Private).map_err(MongoSerializeBsonError)?
        })
        .await
        .map_err(MongoQueryError)?;

        match message_doc {
            Some(doc) => Ok(SingleMessageResponse {
                status: "success",
                message: self.doc_to_message(&doc),
            }),
            None => Err(NotFoundError(id.to_string())),
        }
    }

    pub async fn get_message(&self, id: &str) -> Result<MessageResponse, MyError> {
    let oid = ObjectId::parse_str(id).map_err(|_| InvalidIDError(id.to_owned()))?;
    
    let message_doc = self.collection
        .find_one(doc! {"_id": oid})
        .await
        .map_err(MongoQueryError)?;
    
    match message_doc {
        Some(doc) => Ok(self.doc_to_message(&doc)),
        None => Err(NotFoundError(id.to_string())),
    }
}

    pub async fn edit_message(
        &self,
        id: &str,
        body: UpdateMessageSchema,
    ) -> Result<MessageModel, String> {
        let oid = ObjectId::parse_str(id).map_err(|_| "Invalid ID".to_string())?;

        let update_doc = doc! {
            "$set": {
                "content": &body.content,
                "updatedAt": chrono::Utc::now(),
                "edited": true
            }
        };

        self.collection
            .find_one_and_update(doc! {"_id": oid}, update_doc)
            .await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Message not found".to_string())
    }

    pub async fn delete_message(&self, id: &str) -> Result<(), String> {
        let oid = ObjectId::parse_str(id).map_err(|_| "Invalid ID".to_string())?;

        let result = self
            .collection
            .delete_one(doc! {"_id": oid})
            .await
            .map_err(|e| e.to_string())?;

        if result.deleted_count == 0 {
            Err("Message not found".to_string())
        } else {
            Ok(())
        }
    }

    fn doc_to_message(&self, message: &MessageModel) -> MessageResponse {
      MessageResponse {
          id: message.id.to_hex(),
          owner: message.owner.clone(),
          content: message.content.clone(),
          message_type: message.message_type.clone(),
          location: message.location.clone(),
          location_id: message.location_id.clone(),
          edited: message.edited,
          createdAt: message.createdAt,
          updatedAt: message.updatedAt,
      }
  }
}
