use crate::api::models::ServiceId;
use bitflags::bitflags;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::hash::Hash;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rights {
    pub user: UserRights,
    pub token: TokenRights,
    pub service: ServiceRights,
}

impl Rights {
    pub fn full() -> Self {
        Self {
            user: UserRights::full(),
            token: TokenRights::full(),
            service: ServiceRights::full(),
        }
    }

    pub fn is_equal_or_less(&self, another: &Rights) -> bool {
        (self.token.flags | another.token.flags) == self.token.flags
            && (self.user.flags | another.user.flags) == self.user.flags
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
pub struct ServiceRights {
    pub scope: RightsScope<ServiceId, ServiceRightsFlags>,
}

impl ServiceRights {
    pub fn full() -> Self {
        Self {
            scope: RightsScope::Everything(ServiceRightsFlags::all()),
        }
    }

    pub fn can_create_tokens_for_service(&self, service_id: &ServiceId) -> bool {
        match &self.scope {
            RightsScope::Everything(rights) => rights.contains(ServiceRightsFlags::WEBHOOK_WRITE),
            RightsScope::Some(services) => match services.get(service_id) {
                None => false,
                Some(rights) => rights.contains(ServiceRightsFlags::WEBHOOK_WRITE),
            },
            RightsScope::None => false,
        }
    }

    pub fn can_delete_tokens_for_service(&self, service_id: &ServiceId) -> bool {
        match &self.scope {
            RightsScope::Everything(rights) => rights.contains(ServiceRightsFlags::WEBHOOK_DELETE),
            RightsScope::Some(services) => match services.get(service_id) {
                None => false,
                Some(rights) => rights.contains(ServiceRightsFlags::WEBHOOK_DELETE),
            },
            RightsScope::None => false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RightsScope<T, B>
where
    T: Sized + Ord,
    B: Sized + Ord,
{
    Everything(B),
    Some(BTreeMap<T, B>),
    None,
}

impl<T, B> Default for RightsScope<T, B>
where
    T: Sized + Ord,
    B: Sized + Ord,
{
    fn default() -> Self {
        RightsScope::None
    }
}

bitflags! {
    #[derive(Serialize, Deserialize)]
    pub struct ServiceRightsFlags: u64 {
        /// Can create a webhook.
        const WEBHOOK_WRITE = (1 << 0);
        /// Can delete a webhook.
        const WEBHOOK_DELETE = (1 << 1);
    }
}
