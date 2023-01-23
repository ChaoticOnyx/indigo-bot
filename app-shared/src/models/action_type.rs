use serde::{Deserialize, Serialize};

use super::{ByondCkey, RoleId, SS14Guid};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionType {
    AccountCreated,
    ByondConnected { ckey: ByondCkey },
    SS14Connected { ss14_guid: SS14Guid },
    RoleAdded { role_id: RoleId },
    RoleRemoved { role_id: RoleId },
    WebhookDeleted,
    WebhookCreated,
    ApiTokenCreated,
    ApiTokenDeleted,
}
