use bitflags::bitflags;
use serde::{Deserialize, Serialize};

use super::RightsFlags;
use app_macros::RightsFlags;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRights {
    pub flags: UserRightsFlags,
}

impl UserRights {
    pub fn full() -> Self {
        Self {
            flags: UserRightsFlags::all(),
        }
    }

    pub fn has_more_or_equal_rights_than(&self, another: &Self) -> bool {
        self.flags.has_more_or_equal_rights_than(&another.flags)
    }
}

bitflags! {
    #[derive(Serialize, Deserialize, RightsFlags)]
    pub struct UserRightsFlags: u64 {
        /// Can get connected accounts.
        const CONNECT_READ = (1 << 0);
        /// Can connect account.
        const CONNECT_WRITE = (1 << 1);
    }
}
