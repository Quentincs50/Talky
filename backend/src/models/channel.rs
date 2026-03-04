use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, FromRow, Serialize)]
pub struct Channel {
    pub id_chan: Uuid,
    pub name_chan: String,
    pub id_serv: Uuid,
    pub created_at: DateTime<Utc>,
}
