pub mod auth;

use axum::{Router};
use axum::routing::{get, post, put};

use crate::dto::user::{CreateUser, LoginResponse, UserLogin};
use utoipa::OpenApi;
use utoipa::openapi::security::{SecurityScheme, Http, HttpAuthScheme};

use crate::routes::auth::auth::{signup, login, logout, me, update_me};

#[derive(OpenApi)]
#[openapi(
    paths(
        auth::signup,
        auth::login,
        auth::logout,
        auth::me
    ),
    components(schemas(CreateUser, UserLogin, LoginResponse)),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "bearer_auth",
            SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
        );
    }
}

pub fn router() -> Router {
    Router::new()
        .route("/auth/signup", post(signup))
        .route("/auth/login", post(login))
        .route("/auth/logout", post(logout))
        .route("/auth/me", get(me))
        .route("/auth/profile/{id}", get(crate::routes::auth::auth::get_user_by_id))
        .route("/auth/me", put(update_me))
}
