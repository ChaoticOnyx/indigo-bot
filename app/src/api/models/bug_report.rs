use crate::prelude::*;
use octocrab::models::IssueId;

#[derive(Debug, Clone, Copy)]
pub struct BugReportDescriptor(pub IssueId);

#[derive(Debug, Clone, Copy)]
pub struct BugReport {
    pub descriptor: BugReportDescriptor,
    pub author_id: discord::id::UserId,
}

impl BugReport {
    pub fn new(author_id: discord::id::UserId, issue_id: IssueId) -> Self {
        Self {
            descriptor: BugReportDescriptor(issue_id),
            author_id,
        }
    }
}