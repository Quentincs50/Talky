use serde::{Deserialize, Serialize};
use validator::Validate;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::openapi::types::DateTimeUtc;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateInvitation {
    pub expires_at: Option<DateTimeUtc>,
    #[validate(range(min = 1))]
    pub max_uses: Option<i32>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct InvitationResponse {
    pub id_inv: Uuid,
    pub code: String,
    pub id_serv: Uuid,
    pub expires_at: Option<DateTimeUtc>,
    pub max_uses: Option<i32>,
    pub uses_count: i32,
}
