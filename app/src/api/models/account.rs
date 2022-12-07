use crate::prelude::*;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Account {
    pub id: u64,
    pub discord_id: discord::id::UserId,
    pub byond_ckey: Option<byond::UserId>,
    pub ss14_guid: Option<ss14::UserId>,
    pub created_at: DateTime<Utc>,
}
