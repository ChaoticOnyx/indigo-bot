use crate::prelude::*;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct NewAccount {
    pub discord_id: discord::id::UserId,
    pub created_at: DateTime<Utc>,
}
