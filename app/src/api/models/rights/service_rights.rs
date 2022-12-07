use super::{RightsFlags, RightsScope};
use crate::api::models::ServiceId;
use bitflags::bitflags;
use serde::{Deserialize, Serialize};

use app_macros::RightsFlags;

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

    pub fn has_more_or_equal_rights_than(&self, another: &Self) -> bool {
        match (&self.scope, &another.scope) {
            (RightsScope::None, RightsScope::None) => true,
            (RightsScope::Everything(_), RightsScope::None) => true,
            (RightsScope::Everything(flags), RightsScope::Everything(another_flags)) => {
                flags.has_more_or_equal_rights_than(another_flags)
            }
            (RightsScope::Some(_), RightsScope::None) => true,
            (RightsScope::Some(scope), RightsScope::Some(another_scope)) => {
                scope
                    .keys()
                    .all(|service_id| another_scope.contains_key(service_id))
                    && another_scope
                        .iter()
                        .all(|(another_service_id, another_flags)| {
                            let flags = scope.get(another_service_id).unwrap();

                            flags.has_more_or_equal_rights_than(another_flags)
                        })
            }
            (_, _) => false,
        }
    }
}

bitflags! {
    #[derive(Serialize, Deserialize, RightsFlags)]
    pub struct ServiceRightsFlags: u64 {
        /// Can create a webhook.
        const WEBHOOK_WRITE = (1 << 0);
        /// Can delete a webhook.
        const WEBHOOK_DELETE = (1 << 1);
    }
}
