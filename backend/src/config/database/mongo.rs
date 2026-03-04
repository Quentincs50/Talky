use mongodb::{
    Client,
    Database
};

use std::env;


pub async fn create_client() -> Result<Database, mongodb::error::Error> {
    let mongo_url =  env::var("MONGODB_URL")
        .expect("DATABASE_URL must be set");

    let db_name = env::var("MONGO_INITDB_DATABASE")
        .expect("MONGO_INITDB_DATABASE must be set");

    let client = Client::with_uri_str(&mongo_url)
        .await?;
    let db = client.database(&db_name);

    db.run_command(mongodb::bson::doc! { "ping": 1 }).await?;

    tracing::info!("MongoDB connecté");
    Ok(db)
}

