use axum::{
    extract::{FromRequestParts, Path},
    http::{request::Parts, StatusCode},
    Extension, Json,
};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use sqlx::PgPool;
use validator::Validate;
use jsonwebtoken::{decode, encode, Header, EncodingKey, DecodingKey, Validation};
use crate::dto::user::{CreateUser, UserLogin, LoginResponse};
use crate::services::user_services;
use std::future::Future;
use std::pin::Pin;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize, 
}

#[utoipa::path(
    post,
    path = "/signup",
    tag = "auth",
    request_body = CreateUser,
    responses(
        (status = 201, description = "User added"),
        (status = 400, description = "Invalid request")
    )
)]
pub async fn signup(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    if let Err(e) = payload.validate() {
        return Err((StatusCode::BAD_REQUEST, e.to_string()));
    }

    match user_services::create_user(&pool, payload).await {
        Ok(user) => Ok(Json(serde_json::json!({
            "id": user.id_user,
            "username": user.username,
            "email": user.email
        }))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
    }
}

#[utoipa::path(
    post,
    path = "/login",
    tag = "auth",
    request_body = UserLogin,
    responses(
        (status = 200, description = "Login successful"),
        (status = 401, description = "Invalid credentials")
    )
)]
pub async fn login(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<UserLogin>,
) -> Result<Json<LoginResponse>, (StatusCode, String)> {
    let user_opt = user_services::authenticate_user(&pool, &payload.email, &payload.password)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    let user = user_opt.ok_or((StatusCode::UNAUTHORIZED, "Invalid email or password".to_string()))?;

    let exp = (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize;

    let claims = Claims {
        sub: user.id_user.to_string(),
        exp,
    };

    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());
    
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()))
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Token error".to_string()))?;

    Ok(Json(LoginResponse {
        token,
        id_user: user.id_user,
        username: user.username,
    }))
}

pub struct AuthUser(pub Claims);

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    fn from_request_parts<'a, 'b>(
        parts: &'a mut Parts,
        _state: &'b S,
    ) -> Pin<Box<dyn Future<Output = Result<Self, Self::Rejection>> + Send + 'a>> 
    {
        let token = parts.headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .map(|v| v.to_string());

        Box::pin(async move {
            let token = token.ok_or((StatusCode::UNAUTHORIZED, "Missing token".to_string()))?;
            
            let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());
            
            let decoded = decode::<Claims>(
                &token, 
                &DecodingKey::from_secret(secret.as_bytes()), 
                &Validation::default()
            ).map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token".to_string()))?;

            Ok(AuthUser(decoded.claims))
        })
    }
}

#[utoipa::path(
      post,
      path = "/logout",
      tag = "auth",
      responses((status = 200, description = "Logged out"))
  )] 
pub async fn logout() -> Json<serde_json::Value> {
      Json(serde_json::json!({ "message": "Logged out" }))
}

#[utoipa::path(
      get,
      path = "/me",
      tag = "auth",
      security(("bearer_auth" = [])),
      responses(
        (status = 200, description = "User info"),
        (status = 401, description = "access denied")
      )
  )]
pub async fn me(
    Extension(pool): Extension<PgPool>,
    AuthUser(claims): AuthUser,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid user id".to_string()))?;

    let user = user_services::get_user(&pool, user_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "User not found".to_string()))?;

    Ok(Json(serde_json::json!({
        "id": user.id_user,
        "username": user.username,
        "email": user.email,
        "status": user.status
    })))
}

#[utoipa::path(
    put,
    path = "/me",
    tag = "auth",
    security(("bearer_auth" = [])),
    request_body = serde_json::Value,
    responses(
        (status = 200, description = "User updated"),
        (status = 401, description = "Sorry, Could not update user")
    )
)]
pub async fn update_me(
    Extension(pool): Extension<PgPool>,
    AuthUser(claims): AuthUser,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid user id".to_string()))?;

    let username = payload.get("username").and_then(|v| v.as_str()).unwrap_or("");
    let email = payload.get("email").and_then(|v| v.as_str()).unwrap_or("");
    let status = payload.get("status").and_then(|v| v.as_str()).unwrap_or("");

    let updated_user = user_services::update_user(&pool, user_id, username, email, status)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(serde_json::json!({
        "id": updated_user.id_user,
        "username": updated_user.username,
        "email": updated_user.email,
        "status": updated_user.status
    })))
}

pub async fn get_user_by_id(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let user = user_services::get_user(&pool, id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "User not found".to_string()))?;

    Ok(Json(serde_json::json!({
        "id": user.id_user,
        "username": user.username,
        "email": user.email,
        "status": user.status
    })))
}