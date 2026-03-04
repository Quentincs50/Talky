use axum::{Router, routing::get, Extension};
use tower::layer;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tracing_subscriber;
use tokio::sync::broadcast;
mod config;
mod routes;
mod models;
mod dto;
mod services;
mod tests;

pub mod openapi;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    dotenvy::dotenv().ok();
    tracing::info!("Démarrage de Talky");

   
    let pg_pool: sqlx::PgPool = config::database::postgre::create_pool().await?;
    config::database::postgre::run_migrations(&pg_pool).await?;
    config::database::postgre::health_check(&pg_pool).await?;

    
    let mongo_db: mongodb::Database = config::database::mongo::create_client().await?;
    tracing::info!("Toutes les connexions sont OK !");

    let cors = CorsLayer::permissive();

    let (tx, _rx) = broadcast::channel::<String>(100);
    
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health_check))
        .merge(routes::router()) 
        .layer(cors)
        .layer(Extension(pg_pool.clone()))
        .layer(Extension(mongo_db.clone()))
        .layer(Extension(tx.clone()));

    
    let port = std::env::var("SERVER_PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()?;
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    tracing::info!("Serveur lancé sur http://{}", addr);
    tracing::info!("Testez: http://localhost:{}/health", port);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}


#[derive(Clone)]
struct AppState {
    pg_pool: sqlx::PgPool,
    mongo_db: mongodb::Database,
}


async fn root() -> &'static str {
    "API Talky "
}

async fn health_check() -> &'static str {
    "OK"
}
