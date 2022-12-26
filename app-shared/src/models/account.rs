use crate::models::RoleId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Hash, Ord, PartialOrd, Eq, PartialEq)]
#[serde(transparent)]
pub struct AccountId(pub i64);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: AccountId,
    pub username: String,
    pub avatar_url: String,
    pub created_at: DateTime<Utc>,
    pub roles: Vec<RoleId>,
}
