use axum::{
    Extension, Json,
    http::StatusCode,
    extract::Path
};
use uuid::Uuid;
use sqlx::PgPool;
use validator::Validate;
use crate::openapi::types::DateTimeUtc;

use crate::routes::auth::auth::AuthUser;
use crate::dto::invitations_dto::{CreateInvitation, InvitationResponse};

#[utoipa::path(
    post,
    path = "/{server_id}/invitations",
    tag = "invitations",
    security(("bearer_auth" = [])),
    request_body = CreateInvitation,
    responses((status = 201, body = InvitationResponse))
)]
pub async fn create_invitation(
    Extension(pool): Extension<PgPool>,
    AuthUser(user): AuthUser,
    Path(server_id): Path<Uuid>,
    Json(payload): Json<CreateInvitation>,
) -> Result<Json<InvitationResponse>, (StatusCode, String)> {

    payload
        .validate()
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    let user_id = Uuid::parse_str(&user.sub)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid user".into()))?;

    let invitation = crate::services::invitations_services::create_invitation(
        &pool,
        user_id,
        server_id,
        payload,
    )
    .await
    .map_err(|e| (StatusCode::FORBIDDEN, e.to_string()))?;

    Ok(Json(InvitationResponse {
        id_inv: invitation.id_inv,
        code: invitation.code,
        id_serv: invitation.id_serv,
        expires_at: invitation.expires_at.map(DateTimeUtc),
        max_uses: invitation.max_uses,
        uses_count: invitation.uses_count,
    }))
}


#[utoipa::path(
    post,
    path = "/{code}/join",
    tag = "invitations",
    security(("bearer_auth" = []))
)]
pub async fn join_with_invite(
    Extension(pool): Extension<PgPool>,
    AuthUser(user): AuthUser,
    Path(code): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {

    let user_id = Uuid::parse_str(&user.sub)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid user".into()))?;

    let server_id = crate::services::invitations_services::join_with_invitation(
        &pool,
        user_id,
        &code,
    )
    .await
    .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(serde_json::json!({
        "message": "joined server",
        "server_id": server_id
    })))
}

