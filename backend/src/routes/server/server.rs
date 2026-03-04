use axum::{
    Extension, Json,
    http::StatusCode,
    extract::Path,
};
use uuid::Uuid;
use sqlx::PgPool;
use validator::Validate;

use crate::routes::auth::auth::AuthUser;
use crate::dto::server_dto::UpdateMemberRole;
use crate::dto::server_dto::{CreateServer, ServerResponse};
use crate::services::server_services;
use crate::services::permissions::Role;
use crate::models::{server::Server, user::User};
use crate::dto::server_member::ServerMember;



#[utoipa::path(
    post,
    path = "/createServer",
    tag = "server",
    security(("bearer_auth" = [])),
    request_body = CreateServer,
    responses((status = 201, body = ServerResponse))
)]
pub async fn create_server(
    Extension(pool): Extension<PgPool>,
    AuthUser(user): AuthUser,
    Json(payload): Json<CreateServer>,
) -> Result<(StatusCode, Json<Server>), (StatusCode, String)> {
    payload.validate().map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

     let id_user = Uuid::parse_str(&user.sub)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid user id".into()))?;


    let server = server_services::create_server(&pool, id_user, payload)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::CREATED, Json(server)))
}



#[utoipa::path(
    get,
    path = "/listServers",
    tag = "server",
    security(("bearer_auth" = []))
)]
pub async fn list_servers(
    Extension(pool): Extension<PgPool>,
    AuthUser(user): AuthUser,
) -> Result<Json<Vec<Server>>, (StatusCode, String)> {

    let id_user = Uuid::parse_str(&user.sub)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid user id".into()))?;

    let servers = server_services::list_user_servers(&pool, id_user)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(servers))
}



#[utoipa::path(
    post,
    path = "/{server_id}/join",
    tag = "server",
    security(("bearer_auth" = []))
)]
pub async fn join_server(
    Extension(pool): Extension<PgPool>,
    AuthUser(user): AuthUser,
    Path(server_id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {

    let id_user = Uuid::parse_str(&user.sub)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid user id".into()))?;

    server_services::join_server(&pool, id_user, server_id)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}



#[utoipa::path(
    delete,
    path = "/{server_id}/leave",
    tag = "server",
    security(("bearer_auth" = []))
)]
pub async fn leave_server(
    Extension(pool): Extension<PgPool>,
    AuthUser(user): AuthUser,
    Path(server_id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {

    let id_user = Uuid::parse_str(&user.sub)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid user id".into()))?;

    server_services::leave_server(&pool, id_user, server_id)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}



#[utoipa::path(
    get,
    path = "/{server_id}/members",
    tag = "server",
    security(("bearer_auth" = []))
)]
pub async fn list_server_members(
    Extension(pool): Extension<PgPool>,
    AuthUser(user): AuthUser,
    Path(server_id): Path<Uuid>,
) -> Result<Json<Vec<ServerMember>>, (StatusCode, String)> {
    let id_user = Uuid::parse_str(&user.sub)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid user id".into()))?;

    let members = server_services::list_server_members(&pool, id_user, server_id)
        .await
        .map_err(|e| (StatusCode::FORBIDDEN, e.to_string()))?;

    Ok(Json(members))
}



#[utoipa::path(
    patch,
    path = "/{server_id}/members/{id_user}/role",
    tag = "server",
    security(("bearer_auth" = []))
)]
pub async fn update_member_role(
    Extension(pool): Extension<PgPool>,
    AuthUser(user): AuthUser,
    Path((server_id, target_id_user)): Path<(Uuid, Uuid)>,
    Json(payload): Json<UpdateMemberRole>,
) -> Result<StatusCode, (StatusCode, String)> {
    let id_user = Uuid::parse_str(&user.sub)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid user id".into()))?;

    server_services::update_member_role(
        &pool,
        id_user,
        server_id,
        target_id_user,
        payload.role,
    )
    .await
    .map_err(|e| (StatusCode::FORBIDDEN, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}



#[utoipa::path(
    delete,
    path = "/deleteServer/{server_id}",
    tag = "server",
    security(("bearer_auth" = []))
)]
pub async fn delete_server(
    Extension(pool): Extension<PgPool>,
    AuthUser(user): AuthUser,
    Path(server_id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    let id_user = Uuid::parse_str(&user.sub)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid user id".into()))?;

    server_services::delete_server(&pool, id_user, server_id)
        .await
        .map_err(|e| (StatusCode::FORBIDDEN, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}



#[utoipa::path(
    post,
    path = "/{server_id}/transfer/{new_owner_id}",
    tag = "server",
    security(("bearer_auth" = []))
)]
pub async fn transfer_ownership(
    Extension(pool): Extension<PgPool>,
    AuthUser(user): AuthUser,
    Path((server_id, new_owner_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, (StatusCode, String)> {
    let id_user = Uuid::parse_str(&user.sub)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid user id".into()))?;
    server_services::transfer_server_ownership(
        &pool,
        id_user,
        server_id,
        new_owner_id,
    )
    .await
    .map_err(|e| (StatusCode::FORBIDDEN, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}
