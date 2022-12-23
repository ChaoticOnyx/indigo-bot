use bitflags::bitflags;
use serde::{Deserialize, Serialize};

bitflags! {
    #[derive(Serialize, Deserialize)]
    #[serde(transparent)]
    pub struct TokenRights: u64 {
        /// Может создавать токены с правами не больше чем у самого себя.
        const TOKEN_CREATE = (1 << 0);
        /// Может удалять токены с правами не больше чем у самого себя.
        const TOKEN_DELETE = (1 << 1);
        /// Может создавать сервисные токены с правами не больше чем у самого себя.
        const SERVICE_TOKEN_CREATE = (1 << 2);
        /// Может удалять сервисные токены с правами не больше чем у самого себя.
        const SERVICE_TOKEN_DELETE = (1 << 3);
    }
}
