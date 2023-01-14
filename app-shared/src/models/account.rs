use std::hash::Hash;

use crate::models::{AccountIntegrations, Role};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::DonationTier;

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Hash, Ord, PartialOrd, Eq, PartialEq)]
#[serde(transparent)]
pub struct AccountId(pub i64);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: AccountId,
    pub username: String,
    pub avatar_url: String,
    pub created_at: DateTime<Utc>,
    pub roles: Vec<Role>,
    pub integrations: AccountIntegrations,
    pub donation_tier: Option<DonationTier>,
}

impl PartialEq for Account {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Account {}

impl Hash for Account {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
