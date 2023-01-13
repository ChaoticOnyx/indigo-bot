use crate::models::{AccountId, Rights, Secret};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiToken {
    pub secret: Secret,
    pub expiration: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub rights: Rights,
    pub creator: Option<AccountId>,
    pub is_service: bool,
}

impl ApiToken {
    pub fn new(
        secret: Secret,
        rights: Rights,
        creator: Option<AccountId>,
        duration: Option<Duration>,
        is_service: bool,
        custom_creation_time: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            secret,
            rights,
            expiration: duration.map(|duration| Utc::now() + duration),
            created_at: custom_creation_time.unwrap_or_else(Utc::now),
            creator,
            is_service,
        }
    }

    pub fn is_expired(&self) -> bool {
        let Some(expiration) = self.expiration else {
            return false;
        };

        Utc::now() > expiration
    }
}
