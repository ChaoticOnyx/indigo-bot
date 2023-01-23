use serde::{Deserialize, Serialize};

use super::AccountId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Actor {
    System,
    User(AccountId),
    Webhook(String),
}
