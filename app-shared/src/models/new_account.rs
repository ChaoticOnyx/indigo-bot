use crate::prelude::*;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct NewAccount {
    pub discord_id: DiscordUserId,
    pub created_at: DateTime<Utc>,
}
