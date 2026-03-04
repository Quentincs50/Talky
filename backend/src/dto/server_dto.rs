use serde::{Deserialize, Serialize};
use validator::Validate;
use utoipa::ToSchema;
use crate::services::permissions::Role;
use crate::models::Server;
use uuid::Uuid;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateServer {
    #[validate(length(min = 3, max = 100))]
    pub name: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ServerResponse {
    pub id_serv: Uuid,
    pub name: String,
    pub owner_id: Uuid,
}





#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateMemberRole {
    pub id_user: Uuid,
    pub role: Role,
}

impl From<Server> for ServerResponse {
    fn from(server: Server) -> Self {
        Self {
            id_serv: server.id_serv,
            name: server.name_serv,
            owner_id: server.owner_id,
            //created_at: server.created_at,
        }
    }
}
