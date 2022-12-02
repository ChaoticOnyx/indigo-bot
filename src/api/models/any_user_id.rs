use crate::prelude::*;

#[derive(Debug, Clone)]
pub enum AnyUserId {
    DiscordId(discord::id::UserId),
    ByondCkey(byond::UserId),
    SS14Guid(ss14::UserId),
    InternalId(u64),
}
