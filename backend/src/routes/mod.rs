pub mod test;
pub mod auth;
pub mod channel;
pub mod message;
pub mod server;
pub mod invitation;
pub mod ws;

use axum::Router;

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(utoipa::OpenApi)]                                                                                                                                                           
  #[openapi(                                                                                                                                                                           
      nest(                                                                                                                                                                            
          (path = "/auth", api = auth::ApiDoc),                                                                                                                                     
          (path = "/channel", api = channel::ApiDoc),                                                                                                                                  
          (path = "/api", api = message::ApiDoc),                                                                                                                                  
          (path = "/server", api = server::ApiDoc),
          (path = "/invitations", api = invitation::ApiDoc),                                                                                                                                     
      )                                                                                                                                                                                
)]                                                                                                                                                                                   
struct ApiDoc;

pub fn router() -> Router {
    Router::new()
        .merge(auth::router())
        .merge(channel::router())
        .merge(message::router())
        .merge(server::router())
        .merge(invitation::router())
        .merge(ws::router())
        .merge(SwaggerUi::new("/swagger-ui")
            .url("/api-docs/openapi.json", ApiDoc::openapi()))
}

