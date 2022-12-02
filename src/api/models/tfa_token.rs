use crate::api::models::TokenSecret;
use crate::prelude::*;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TFAToken {
    pub user: discord::user::User,
    pub secret: TokenSecret,
    pub expiration: DateTime<Utc>,
}

impl TFAToken {
    pub fn new(secret: TokenSecret, user: discord::user::User, duration: Duration) -> Self {
        Self {
            user,
            secret,
            expiration: Utc::now() + duration,
        }
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expiration
    }
}
