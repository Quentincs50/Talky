use sqlx::PgPool;
use uuid::Uuid;
use crate::dto::channel_dto::CreateChannel;
use crate::models::channel::Channel;
use crate::services::permissions::{get_user_role, is_admin};
use crate::dto::channel_dto::UpdateChannel;
use crate::services::permissions::is_member;


pub async fn create_channel(
    pool: &PgPool,
    id_user: Uuid,
    server_id: Uuid,
    req: CreateChannel,
) -> Result<Channel, sqlx::Error> {

    let role = get_user_role(pool, id_user, server_id).await?;
    if !is_admin(role) {
        return Err(sqlx::Error::Protocol("Forbidden".into()));
    }

    let channel = sqlx::query_as::<_, Channel>(
        r#"
        INSERT INTO channels (id_chan, name_chan, id_serv)
        VALUES ($1, $2, $3)
        RETURNING id_chan, name_chan, id_serv, created_at
        "#
    )
    .bind(Uuid::new_v4())
    .bind(&req.name)
    .bind(server_id)
    .fetch_one(pool)
    .await?;

    Ok(channel)
}


pub async fn list_channels(
    pool: &PgPool,
    id_user: Uuid,
    server_id: Uuid,
) -> Result<Vec<Channel>, sqlx::Error> {

    let role = get_user_role(pool, id_user, server_id).await?;
    if !is_member(role) {
        return Err(sqlx::Error::Protocol("Forbidden".into()));
    }

    let channels = sqlx::query_as::<_, Channel>(
        r#"
        SELECT id_chan, name_chan, id_serv, created_at
        FROM channels
        WHERE id_serv = $1
        ORDER BY created_at ASC
        "#
    )
    .bind(server_id)
    .fetch_all(pool)
    .await?;

    Ok(channels)
}



pub async fn update_channel(
    pool: &PgPool,
    id_user: Uuid,
    channel_id: Uuid,
    req: UpdateChannel,
) -> Result<(), sqlx::Error> {

    let server_id = get_server_id_by_channel(pool, channel_id).await?;

    let role = get_user_role(pool, id_user, server_id).await?;
    if !is_admin(role) {
        return Err(sqlx::Error::Protocol("Forbidden".into()));
    }

    sqlx::query(
        "UPDATE channels SET name_chan = $1 WHERE id_chan = $2"
    )
    .bind(&req.name)
    .bind(channel_id)
    .execute(pool)
    .await?;

    Ok(())
}


pub async fn delete_channel(
    pool: &PgPool,
    id_user: Uuid,
    channel_id: Uuid,
    server_id: Uuid,
) -> Result<(), sqlx::Error> {

    let role = get_user_role(pool, id_user, server_id).await?;
    if !is_admin(role) {
        return Err(sqlx::Error::Protocol("Forbidden".into()));
    }

    sqlx::query(
        "DELETE FROM channels WHERE id_chan = $1 AND id_serv = $2"
    )
    .bind(channel_id)
    .bind(server_id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_channel_by_id(
    pool: &PgPool,
    id_user: Uuid,
    channel_id: Uuid,
) -> Result<Channel, sqlx::Error> {

    let channel = sqlx::query_as::<_, Channel>(
        r#"
        SELECT id_chan, name_chan, id_serv, created_at
        FROM channels
        WHERE id_chan = $1
        "#
    )
    .bind(channel_id)
    .fetch_one(pool)
    .await?;

    let role = get_user_role(pool, id_user, channel.id_serv).await?;
    if !crate::services::permissions::is_member(role) {
        return Err(sqlx::Error::Protocol("Forbidden".into()));
    }

    Ok(channel)
}

pub async fn get_server_id_by_channel(
    pool: &PgPool,
    channel_id: Uuid,
) -> Result<Uuid, sqlx::Error> {
    sqlx::query_scalar(
        "SELECT id_serv FROM channels WHERE id_chan = $1"
    )
    .bind(channel_id)
    .fetch_one(pool)
    .await
}




