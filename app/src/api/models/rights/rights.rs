use crate::api::models::{ServiceRights, TokenRights, UserRights};
use serde::{Deserialize, Serialize};

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

    pub fn has_more_or_equal_rights_than(&self, another: &Rights) -> bool {
        self.token.has_more_or_equal_rights_than(&another.token)
            && self.user.has_more_or_equal_rights_than(&another.user)
            && self.service.has_more_or_equal_rights_than(&another.service)
    }
}
