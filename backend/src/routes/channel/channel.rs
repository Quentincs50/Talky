use axum::{
    Extension, Json,
    http::StatusCode,
    extract::Path,
};
use uuid::Uuid;
use sqlx::PgPool;
use validator::Validate;

use crate::routes::auth::auth::AuthUser;
use crate::dto::channel_dto::{CreateChannel,UpdateChannel, ChannelResponse};

#[utoipa::path(
    post,
    path = "/{server_id}/channels",
    tag = "channel",
    security(("bearer_auth" = [])),
    request_body = CreateChannel,
    responses(
        (status = 201, description = "Channel created", body = ChannelResponse),
        (status = 403, description = "Forbidden")
    )
)]
pub async fn create_channel(
    Extension(pool): Extension<PgPool>,
    AuthUser(user): AuthUser,
    Path(server_id): Path<Uuid>,
    Json(payload): Json<CreateChannel>,
) -> Result<(StatusCode, Json<ChannelResponse>), (StatusCode, String)> {
    payload.validate().map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    let id_user = Uuid::parse_str(&user.sub)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid user id".into()))?;

    let channel = crate::services::channel_services::create_channel(&pool, id_user, server_id, payload)
        .await
        .map_err(|e| {
            let msg = e.to_string();
            if msg.contains("Forbidden") {
                (StatusCode::FORBIDDEN, "Forbidden".into())
            } else {
                (StatusCode::INTERNAL_SERVER_ERROR, msg)
            }
        })?;

    Ok((StatusCode::CREATED, Json(ChannelResponse {
        id_chan: channel.id_chan,
        name: channel.name_chan,
        id_serv: channel.id_serv,
    })))
}

#[utoipa::path(
    get,
    path = "/{server_id}/channels",
    tag = "channel",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "List channels", body = [ChannelResponse])
    )
)]

pub async fn list_channels(
    Extension(pool): Extension<PgPool>,
    AuthUser(user): AuthUser,
    Path(server_id): Path<Uuid>,
) -> Result<Json<Vec<ChannelResponse>>, (StatusCode, String)> {

    let id_user = Uuid::parse_str(&user.sub)
    .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid user id".into()))?;


    let channels = crate::services::channel_services::list_channels(
        &pool,
        id_user,
        server_id,
    )
    .await
    .map_err(|e| map_channel_error(e))?;

    Ok(Json(
        channels.into_iter().map(ChannelResponse::from).collect()
    ))
}

#[utoipa::path(
    delete,
    path = "/{channel_id}",
    tag = "channel",
    security(("bearer_auth" = [])),
    responses((status = 204, description = "Deleted"))
)]
pub async fn delete_channel(
    Extension(pool): Extension<PgPool>,
    AuthUser(user): AuthUser,
    Path(channel_id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    let id_user = Uuid::parse_str(&user.sub)
    .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid user id".into()))?;


    crate::services::channel_services::delete_channel(
        &pool,
        id_user,
        channel_id,
        
        crate::services::channel_services::get_channel_by_id(&pool, id_user, channel_id)
            .await
            .map_err(|e| map_channel_error(e))?
            .id_serv,
    )
    .await
    .map_err(|e| map_channel_error(e))?;

    Ok(StatusCode::NO_CONTENT)
}


#[utoipa::path(
    get,
    path = "/{channel_id}",
    tag = "channel",
    security(("bearer_auth" = [])),
    responses((status = 200, body = ChannelResponse))
)]
pub async fn get_channel(
    Extension(pool): Extension<PgPool>,
    AuthUser(user): AuthUser,
    Path(channel_id): Path<Uuid>,
) -> Result<Json<ChannelResponse>, (StatusCode, String)> {
    let id_user = Uuid::parse_str(&user.sub)
    .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid user id".into()))?;


    let channel = crate::services::channel_services::get_channel_by_id(
        &pool,
        id_user,
        channel_id,
    )
    .await
    .map_err(|e| map_channel_error(e))?;

    Ok(Json(channel.into()))
}


#[utoipa::path(
    put,
    path = "/{channel_id}",
    tag = "channel",
    security(("bearer_auth" = [])),
    request_body = UpdateChannel,
    responses((status = 204))
)]
pub async fn update_channel(
    Extension(pool): Extension<PgPool>,
    AuthUser(user): AuthUser,
    Path(channel_id): Path<Uuid>,
    Json(payload): Json<UpdateChannel>,
) -> Result<StatusCode, (StatusCode, String)> {

    payload.validate()
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    let id_user = Uuid::parse_str(&user.sub)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid user id".into()))?;

    crate::services::channel_services::update_channel(
        &pool,
        id_user,
        channel_id,
        payload,
    )
    .await
    .map_err(|e| {
        let msg = e.to_string();
        if msg.contains("Forbidden") {
            (StatusCode::FORBIDDEN, "Forbidden".into())
        } else {
            (StatusCode::INTERNAL_SERVER_ERROR, msg)
        }
    })?;

    Ok(StatusCode::NO_CONTENT)
}





fn map_channel_error(e: sqlx::Error) -> (StatusCode, String) {
    let msg = e.to_string();
    if msg.contains("Forbidden") {
        (StatusCode::FORBIDDEN, msg)
    } else if msg.contains("RowNotFound") {
        (StatusCode::NOT_FOUND, "Not found".into())
    } else {
        (StatusCode::INTERNAL_SERVER_ERROR, msg)
    }
}
