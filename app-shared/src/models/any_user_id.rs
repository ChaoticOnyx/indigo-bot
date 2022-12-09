use crate::{models::AccountId, prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnyUserId {
    DiscordId(DiscordUserId),
    ByondCkey(ByondUserId),
    SS14Guid(SS14UserId),
    AccountId(AccountId),
}
