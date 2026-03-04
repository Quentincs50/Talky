use rand::{distributions::Alphanumeric, Rng};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;
use crate::dto::invitations_dto::CreateInvitation;
use crate::models::invitations::Invitation;
use crate::services::permissions::{get_user_role, is_admin};
use crate::services::permissions::is_member;

fn generate_code() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect()
}


pub async fn create_invitation(
    pool: &PgPool,
    id_user: Uuid,
    server_id: Uuid,
    req: CreateInvitation,
) -> Result<Invitation, sqlx::Error> {

   
    let role = get_user_role(pool, id_user, server_id).await?;
    if !is_admin(role) {
        return Err(sqlx::Error::Protocol("Forbidden".into()));
    }

    
    let code = generate_code();

    
    let expires_at_chrono: Option<chrono::DateTime<Utc>> = req.expires_at.map(|dt| dt.0);

    
    let invitation = sqlx::query_as::<_, Invitation>(
        r#"
        INSERT INTO invitations (
            id_inv, id_serv, code, created_by,
            expires_at, max_uses
        )
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING *
        "#
    )
    .bind(Uuid::new_v4())
    .bind(server_id)
    .bind(code)
    .bind(id_user)
    .bind(expires_at_chrono) 
    .bind(req.max_uses)
    .fetch_one(pool)
    .await?;

    Ok(invitation)
}



pub async fn join_with_invitation(
    pool: &PgPool,
    id_user: Uuid,
    code: &str,
) -> Result<Uuid, sqlx::Error> {

    let mut tx = pool.begin().await?;

    let invitation = sqlx::query_as::<_, Invitation>(
        "SELECT * FROM invitations WHERE code = $1"
    )
    .bind(code)
    .fetch_optional(&mut *tx)
    .await?
    .ok_or(sqlx::Error::RowNotFound)?;

  
    if let Some(exp) = invitation.expires_at {
        if exp < Utc::now() {
            return Err(sqlx::Error::Protocol("Invitation expired".into()));
        }
    }

 
    if let Some(max) = invitation.max_uses {
        if invitation.uses_count >= max {
            return Err(sqlx::Error::Protocol("Invitation exhausted".into()));
        }
    }

  
    let role = get_user_role(pool, id_user, invitation.id_serv).await?;
    if is_member(role) {
        tx.commit().await?;
        return Ok(invitation.id_serv);
        //return Err(sqlx::Error::Protocol("Already member".into()));
    }

  
    sqlx::query(
        r#"
        INSERT INTO server_members (id_user, id_serv, role)
        VALUES ($1, $2, 'MEMBER')
        "#
    )
    .bind(id_user)
    .bind(invitation.id_serv)
    .execute(&mut *tx)
    .await?;

   
    sqlx::query(
        "UPDATE invitations SET uses_count = uses_count + 1 WHERE id_inv = $1"
    )
    .bind(invitation.id_inv)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(invitation.id_serv)
}
