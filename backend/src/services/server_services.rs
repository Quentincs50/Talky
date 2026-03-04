use sqlx::PgPool;
use uuid::Uuid;

use crate::dto::server_dto::CreateServer;
use crate::dto::server_member::ServerMember;
use crate::models::{server::Server, user::User};
use crate::services::permissions::{
    get_user_role,
    is_member,
    is_owner,
    Role,
};


pub async fn create_server(
    pool: &PgPool,
    id_user: Uuid,
    req: CreateServer,
) -> Result<Server, sqlx::Error> {

    let mut tx = pool.begin().await?;

    let server = sqlx::query_as::<_, Server>(
        r#"
        INSERT INTO servers (id_serv, name_serv, owner_id)
        VALUES ($1, $2, $3)
        RETURNING id_serv, name_serv, owner_id, created_at
        "#
    )
    .bind(Uuid::new_v4())
    .bind(&req.name)
    .bind(id_user)
    .fetch_one(&mut *tx)
    .await?;

   
    sqlx::query(
        r#"
        INSERT INTO server_members (id_user, id_serv, role)
        VALUES ($1, $2, 'OWNER')
        "#
    )
    .bind(id_user)
    .bind(server.id_serv)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(server)
}


pub async fn list_user_servers(
    pool: &PgPool,
    id_user: Uuid,
) -> Result<Vec<Server>, sqlx::Error> {

    let servers = sqlx::query_as::<_, Server>(
        r#"
        SELECT s.id_serv, s.name_serv, s.owner_id, s.created_at
        FROM servers s
        JOIN server_members sm ON sm.id_serv = s.id_serv
        WHERE sm.id_user = $1
        ORDER BY s.created_at ASC
        "#
    )
    .bind(id_user)
    .fetch_all(pool)
    .await?;

    Ok(servers)
}


pub async fn join_server(
    pool: &PgPool,
    id_user: Uuid,
    server_id: Uuid,
) -> Result<(), sqlx::Error> {

  
    let role = get_user_role(pool, id_user, server_id).await?;
    if role.is_some() {
        return Err(sqlx::Error::Protocol("Already member".into()));
    }

    sqlx::query(
        r#"
        INSERT INTO server_members (id_user, id_serv, role)
        VALUES ($1, $2, 'MEMBER')
        "#
    )
    .bind(id_user)
    .bind(server_id)
    .execute(pool)
    .await?;

    Ok(())
}


pub async fn leave_server(
    pool: &PgPool,
    id_user: Uuid,
    server_id: Uuid,
) -> Result<(), sqlx::Error> {

    let role = get_user_role(pool, id_user, server_id).await?;
    if is_owner(role) {
        return Err(sqlx::Error::Protocol("Owner cannot leave server".into()));
    }


    sqlx::query(
        "DELETE FROM server_members WHERE id_user = $1 AND id_serv = $2"
    )
    .bind(id_user)
    .bind(server_id)
    .execute(pool)
    .await?;

    Ok(())
}


pub async fn list_server_members(
    pool: &PgPool,
    id_user: Uuid,
    server_id: Uuid,
) -> Result<Vec<ServerMember>, sqlx::Error> {

    let role = get_user_role(pool, id_user, server_id).await?;
    if !is_member(role) {
        return Err(sqlx::Error::Protocol("Forbidden".into()));
    }

    let members = sqlx::query_as::<_, ServerMember>(
        r#"
        SELECT u.id_user, u.username, u.email,
               u.password_hash, u.status, u.created_at, sm.role
        FROM users u
        JOIN server_members sm ON sm.id_user = u.id_user
        WHERE sm.id_serv = $1
        "#
    )
    .bind(server_id)
    .fetch_all(pool)
    .await?;

    Ok(members)
}


pub async fn update_member_role(
    pool: &PgPool,
    owner_id: Uuid,
    server_id: Uuid,
    target_id_user: Uuid,
    new_role: Role,
) -> Result<(), sqlx::Error> {

    let role = get_user_role(pool, owner_id, server_id).await?;
    if !is_owner(role) {
        return Err(sqlx::Error::Protocol(
            "Only owner can update roles".into(),
        ));
    }
    if matches!(new_role, Role::OWNER) {
        return Err(sqlx::Error::Protocol(
            "Use transfer ownership endpoint".into(),
        ));
    }


    sqlx::query(
        r#"
        UPDATE server_members
        SET role = $1::role_type
        WHERE id_user = $2 AND id_serv = $3
        "#
    )
    .bind(new_role.to_string())
    .bind(target_id_user)
    .bind(server_id)
    .execute(pool)
    .await?;

    Ok(())
}


pub async fn delete_server(
    pool: &PgPool,
    id_user: Uuid,
    server_id: Uuid,
) -> Result<(), sqlx::Error> {

    let role = get_user_role(pool, id_user, server_id).await?;
    if !is_owner(role) {
        return Err(sqlx::Error::Protocol("Forbidden".into()));
    }


    sqlx::query(
        "DELETE FROM servers WHERE id_serv = $1"
    )
    .bind(server_id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn transfer_server_ownership(
    pool: &PgPool,
    current_owner_id: Uuid,
    server_id: Uuid,
    new_owner_id: Uuid,
) -> Result<(), sqlx::Error> {

    
    let role = get_user_role(pool, current_owner_id, server_id).await?;
    if !is_owner(role) {
        return Err(sqlx::Error::Protocol(
            "Only owner can transfer ownership".into(),
        ));
    }


    
    let target_role = get_user_role(pool, new_owner_id, server_id).await?;
    if target_role.is_none() {
        return Err(sqlx::Error::Protocol(
            "Target user must be a member".into(),
        ));
    }

    let mut tx = pool.begin().await?;

    
    sqlx::query(
        "UPDATE servers SET owner_id = $1 WHERE id_serv = $2"
    )
    .bind(new_owner_id)
    .bind(server_id)
    .execute(&mut *tx)
    .await?;

    
    sqlx::query(
        r#"
        UPDATE server_members
        SET role = CASE
            WHEN id_user = $1 THEN 'OWNER'
            WHEN id_user = $2 THEN 'ADMIN'
            ELSE role
        END
        WHERE id_serv = $3
        "#
    )
    .bind(new_owner_id)
    .bind(current_owner_id)
    .bind(server_id)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(())
}

