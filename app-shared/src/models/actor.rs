use serde::{Deserialize, Serialize};

use super::AccountId;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum Actor {
    System,
    User(AccountId),
    Webhook(String),
}
