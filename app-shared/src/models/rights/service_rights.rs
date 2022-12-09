use super::RightsScope;
use crate::models::ServiceId;
use bitflags::bitflags;
use serde::{Deserialize, Serialize};

pub type ScopedServiceRights = RightsScope<ServiceId, ServiceRights>;

impl ScopedServiceRights {
    pub fn can_create_tokens_for_service(&self, service_id: &ServiceId) -> bool {
        match &self {
            RightsScope::Everything(rights) => rights.contains(ServiceRights::WEBHOOK_WRITE),
            RightsScope::Some(services) => match services.get(service_id) {
                None => false,
                Some(rights) => rights.contains(ServiceRights::WEBHOOK_WRITE),
            },
            RightsScope::None => false,
        }
    }

    pub fn can_delete_tokens_for_service(&self, service_id: &ServiceId) -> bool {
        match &self {
            RightsScope::Everything(rights) => rights.contains(ServiceRights::WEBHOOK_DELETE),
            RightsScope::Some(services) => match services.get(service_id) {
                None => false,
                Some(rights) => rights.contains(ServiceRights::WEBHOOK_DELETE),
            },
            RightsScope::None => false,
        }
    }

    pub fn all() -> Self {
        Self::Everything(ServiceRights::all())
    }

    pub fn empty() -> Self {
        Self::None
    }
}

bitflags! {
    #[derive(Serialize, Deserialize)]
    #[serde(transparent)]
    pub struct ServiceRights: u64 {
        /// Can create a webhook.
        const WEBHOOK_WRITE = (1 << 0);
        /// Can delete a webhook.
        const WEBHOOK_DELETE = (1 << 1);
    }
}
