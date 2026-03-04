use serde::{Deserialize, Serialize};
use validator::Validate;
use utoipa::ToSchema;
use uuid::Uuid;
use crate::models::channel::Channel;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateChannel {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateChannel {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ChannelResponse {
    pub id_chan: Uuid,
    pub name: String,
    pub id_serv: Uuid,
}



impl From<Channel> for ChannelResponse {
    fn from(channel: Channel) -> Self {
        Self {
            id_chan: channel.id_chan,
            name: channel.name_chan,
            id_serv: channel.id_serv,
        }
    }
}

