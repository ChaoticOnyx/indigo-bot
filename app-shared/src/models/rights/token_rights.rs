use bitflags::bitflags;
use serde::{Deserialize, Serialize};

bitflags! {
    #[derive(Serialize, Deserialize)]
    #[serde(transparent)]
    pub struct TokenRights: u64 {
        /// Can create tokens with no more rights than he has himself.
        const TOKEN_CREATE = (1 << 0);
        /// Can delete tokens with no more rights than he has himself.
        const TOKEN_DELETE = (1 << 1);
        /// Can create service tokens with no more rights than he has himself.
        const SERVICE_TOKEN_CREATE = (1 << 2);
        /// Can delete service tokens with no more rights than he has himself.
        const SERVICE_TOKEN_DELETE = (1 << 3);
    }
}
