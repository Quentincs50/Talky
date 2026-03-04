use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;

pub async fn create_pool() -> Result<PgPool, sqlx::Error> {
    let database_url = env::var("DATABASE_URL")
    .expect("le .env n'a pas database_url");

    tracing::info!("connexion à psql");

    let pool = PgPoolOptions::new()
    .max_connections(10)
    .min_connections(2)
    .connect(&database_url)
    .await?;

    tracing::info!("psql connecté avec succès");

    Ok(pool)
}

pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::Error> {

    
    sqlx::migrate!("./migrations")
        .run(pool)
        .await?;
    
    tracing::info!("Migrations executées");
    Ok(())
}

pub async fn health_check(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query("SELECT 1")
        .fetch_one(pool)
        .await?;
    Ok(())
}