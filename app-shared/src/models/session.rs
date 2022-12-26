use crate::models::{AccountId, Secret};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub secret: Secret,
    pub api_secret: Secret,
    pub csrf_token: Secret,
    pub account_id: AccountId,
    pub created_at: DateTime<Utc>,
    pub expiration: DateTime<Utc>,
    pub user_agent: String,
    pub ip: String,
}

impl Session {
    pub fn new(
        secret: Secret,
        api_secret: Secret,
        csrf_token: Secret,
        account_id: AccountId,
        custom_creation_date: Option<DateTime<Utc>>,
        duration: Duration,
        user_agent: String,
        ip: String,
    ) -> Self {
        Self {
            secret,
            api_secret,
            csrf_token,
            account_id,
            created_at: custom_creation_date.unwrap_or_else(|| Utc::now()),
            expiration: Utc::now() + duration,
            user_agent,
            ip,
        }
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expiration
    }
}
