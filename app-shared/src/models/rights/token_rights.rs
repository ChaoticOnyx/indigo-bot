use super::RightsFlags;
use bitflags::bitflags;
use serde::{Deserialize, Serialize};

use app_macros::RightsFlags;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenRights {
    pub flags: TokenRightsFlags,
}

impl TokenRights {
    pub fn full() -> Self {
        Self {
            flags: TokenRightsFlags::all(),
        }
    }

    pub fn has_more_or_equal_rights_than(&self, another: &Self) -> bool {
        self.flags.has_more_or_equal_rights_than(&another.flags)
    }
}

bitflags! {
    #[derive(Serialize, Deserialize, RightsFlags)]
    pub struct TokenRightsFlags: u64 {
        /// Can create tokens with no more rights than he has himself.
        const TOKEN_CREATE = (1 << 0);
        /// Can delete tokens with no more rights than he has himself.
        const TOKEN_DELETE = (1 << 1);
    }
}
