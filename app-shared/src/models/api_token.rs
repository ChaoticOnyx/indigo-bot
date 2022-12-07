use crate::models::{Rights, Secret};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiToken {
    pub secret: Secret,
    pub expiration: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub rights: Rights,
}

impl ApiToken {
    pub fn new(secret: Secret, rights: Rights, duration: Option<Duration>) -> Self {
        Self {
            secret,
            rights,
            expiration: duration.map(|duration| Utc::now() + duration),
            created_at: Utc::now(),
        }
    }

    pub fn is_expired(&self) -> bool {
        let Some(expiration) = self.expiration else {
            return false;
        };

        Utc::now() > expiration
    }
}
