use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum Role {
    OWNER,
    ADMIN,
    MEMBER,
}

pub async fn get_user_role(
    pool: &PgPool,
    id_user: Uuid,
    server_id: Uuid,
) -> Result<Option<Role>, sqlx::Error> {
    let role = sqlx::query_scalar::<_, String>(
        r#"
        SELECT role::text
        FROM server_members
        WHERE id_user = $1 AND id_serv = $2
        "#
    )
    .bind(id_user)
    .bind(server_id)
    .fetch_optional(pool)
    .await?;

    Ok(role.map(|r| match r.as_str() {
        "OWNER" => Role::OWNER,
        "ADMIN" => Role::ADMIN,
        _ => Role::MEMBER,
    }))
}

pub fn is_member(role: Option<Role>) -> bool {
    role.is_some()
}

pub fn is_admin(role: Option<Role>) -> bool {
    matches!(role, Some(Role::ADMIN | Role::OWNER))
}

pub fn is_owner(role: Option<Role>) -> bool {
    matches!(role, Some(Role::OWNER))
}

impl ToString for Role {
    fn to_string(&self) -> String {
        match self {
            Role::OWNER => "OWNER",
            Role::ADMIN => "ADMIN",
            Role::MEMBER => "MEMBER",
        }
        .to_string()
    }
}

