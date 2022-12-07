use crate::prelude::*;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Account {
    pub id: u64,
    pub discord_id: DiscordUserId,
    pub byond_ckey: Option<ByondUserId>,
    pub ss14_guid: Option<SS14UserId>,
    pub created_at: DateTime<Utc>,
}
