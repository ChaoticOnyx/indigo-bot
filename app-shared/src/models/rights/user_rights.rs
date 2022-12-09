use bitflags::bitflags;
use serde::{Deserialize, Serialize};

bitflags! {
    #[derive(Serialize, Deserialize)]
    #[serde(transparent)]
    pub struct UserRights: u64 {
        /// Can get connected accounts.
        const GET_CONNECTED_ACCOUNTS = (1 << 0);
        /// Can connect account.
        const ADD_CONNECTED_ACCOUNTS = (1 << 1);
        /// Can add roles to users with lower rights.
        const ADD_ROLES = (1 << 2);
        /// Can remove roles from users with lower rights.
        const REMOVE_ROLES = (1 << 2);
    }
}
