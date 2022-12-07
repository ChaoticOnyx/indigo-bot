use crate::prelude::*;

#[derive(Debug, Clone)]
pub enum AnyUserId {
    DiscordId(DiscordUserId),
    ByondCkey(ByondUserId),
    SS14Guid(SS14UserId),
    InternalId(u64),
}
