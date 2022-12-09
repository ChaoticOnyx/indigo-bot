use crate::{models::RoleId, prelude::*};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Hash, Ord, PartialOrd, Eq, PartialEq)]
#[serde(transparent)]
pub struct AccountId(pub i64);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: AccountId,
    pub discord_id: DiscordUserId,
    pub byond_ckey: Option<ByondUserId>,
    pub ss14_guid: Option<SS14UserId>,
    pub created_at: DateTime<Utc>,
    pub roles: Vec<RoleId>,
}
