use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{ActionType, Actor};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[serde(transparent)]
pub struct JournalEntryId(pub i64);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalEntry {
    pub id: JournalEntryId,
    pub object: Actor,
    pub datetime: DateTime<Utc>,
    pub subject: Option<Actor>,
    pub action: ActionType,
}
