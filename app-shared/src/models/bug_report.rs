use crate::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct BugReport {
    pub issue_number: u64,
    pub author_id: DiscordUserId,
}

impl BugReport {
    pub fn new(author_id: DiscordUserId, issue_number: u64) -> Self {
        Self {
            issue_number,
            author_id,
        }
    }
}
