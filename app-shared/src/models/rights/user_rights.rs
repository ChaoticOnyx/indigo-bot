use bitflags::bitflags;
use serde::{Deserialize, Serialize};

bitflags! {
    #[derive(Serialize, Deserialize)]
    #[serde(transparent)]
    pub struct UserRights: u64 {
        /// Может получать интеграции.
        const GET_CONNECTED_ACCOUNTS = (1 << 0);
        /// Может создавать интеграции.
        const ADD_CONNECTED_ACCOUNTS = (1 << 1);
        /// Может добавлять роли пользователям с меньшими правами.
        const ADD_ROLES = (1 << 2);
        /// Может удалять роли пользователям с меньшими правами.
        const REMOVE_ROLES = (1 << 2);
    }
}

impl Default for UserRights {
    fn default() -> Self {
        Self::empty()
    }
}
