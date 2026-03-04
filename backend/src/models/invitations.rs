use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, FromRow, Serialize)]
pub struct Invitation {
    pub id_inv: Uuid,
    pub id_serv: Uuid,
    pub code: String,
    pub created_by: Uuid,
    pub expires_at: Option<DateTime<Utc>>,
    pub max_uses: Option<i32>,
    pub uses_count: i32,
    pub created_at: DateTime<Utc>,
}
