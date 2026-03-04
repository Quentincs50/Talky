use serde::{Deserialize, Serialize};
use validator::Validate;
use utoipa::ToSchema;
use crate::services::permissions::Role;
use crate::models::Server;
use uuid::Uuid;


use sqlx::Type;

#[derive(Debug, Serialize, Deserialize, Type, PartialEq)]
#[sqlx(type_name = "role_type", rename_all = "UPPERCASE")]
pub enum RoleType {
    OWNER,
    ADMIN,
    MEMBER,
}


#[derive(sqlx::FromRow, Serialize)]
pub struct ServerMember {
    pub id_user: Uuid,
    pub username: String,
    pub email: String,
    pub status: String,
    //pub created_at: DateTime<Utc>,
    pub role: RoleType,
}
