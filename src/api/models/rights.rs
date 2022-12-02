use crate::api::models::GameServerId;
use bitflags::bitflags;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rights {
    pub server: GameServerRights,
    pub user: UserRights,
    pub token: TokenRights,
}

impl Rights {
    pub fn full() -> Self {
        Self {
            server: GameServerRights::full(),
            user: UserRights::full(),
            token: TokenRights::full(),
        }
    }
}

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
}

bitflags! {
    #[derive(Serialize, Deserialize)]
    pub struct TokenRightsFlags: u64 {
        /// Can create tokens with no more rights than he has himself.
        const TOKEN_CREATE = (1 << 0);
        /// Can delete tokens with no more rights than he has himself.
        const TOKEN_DELETE = (1 << 1);
    }
}

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
}

bitflags! {
    #[derive(Serialize, Deserialize)]
    pub struct UserRightsFlags: u64 {
        /// Can get connected accounts.
        const CONNECT_READ = (1 << 0);
        /// Can connect account.
        const CONNECT_WRITE = (1 << 1);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameServerRights {
    pub flags: GameServerRightsFlags,
    pub server_ids: Option<Vec<GameServerId>>,
}

impl GameServerRights {
    pub fn full() -> Self {
        Self {
            flags: GameServerRightsFlags::all(),
            server_ids: None,
        }
    }
}

bitflags! {
    #[derive(Serialize, Deserialize)]
    pub struct GameServerRightsFlags: u64 {
        const SERVICES_READ = (1 << 0);
        const SERVICES_WRITE = (1 << 1);
    }
}
