use super::Secret;
use crate::prelude::*;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TFAToken {
    pub discord_user_id: DiscordUserId,
    pub secret: Secret,
    pub expiration: DateTime<Utc>,
}

impl TFAToken {
    pub fn new(secret: Secret, discord_user_id: DiscordUserId, duration: Duration) -> Self {
        Self {
            discord_user_id,
            secret,
            expiration: Utc::now() + duration,
        }
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expiration
    }
}
