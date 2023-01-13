use crate::models::AccountId;
use crate::prelude::{ByondUserId, DiscordUserId, SS14UserId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AccountIntegrations {
    pub account_id: AccountId,
    pub discord_user_id: DiscordUserId,
    pub byond_ckey: Option<ByondUserId>,
    pub ss14_guid: Option<SS14UserId>,
}

impl AccountIntegrations {
    pub fn new(
        account_id: AccountId,
        discord_user_id: DiscordUserId,
        byond_ckey: Option<ByondUserId>,
        ss14_guid: Option<SS14UserId>,
    ) -> Self {
        Self {
            account_id,
            discord_user_id,
            byond_ckey,
            ss14_guid,
        }
    }
}
