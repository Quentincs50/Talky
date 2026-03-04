use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, FromRow, Serialize)]
pub struct Server {
    pub id_serv: Uuid,
    pub name_serv: String,
    pub owner_id: Uuid,
    pub created_at: DateTime<Utc>,
}
