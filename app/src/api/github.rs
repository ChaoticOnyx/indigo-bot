use std::collections::HashSet;

use crate::prelude::*;
use octocrab::{models::IssueId, Octocrab};

#[derive(Debug, Clone)]
pub struct Github {
    client: Octocrab,
}

impl Github {
    pub fn new(token: String) -> Self {
        let client = Octocrab::builder().personal_token(token).build().unwrap();

        Self { client }
    }

    #[instrument(skip(self))]
    pub async fn create_issue(
        &self,
        repository: String,
        title: String,
        body: String,
        labels: HashSet<String>,
    ) -> IssueId {
        info!("create_issue");

        let (owner, repo) = repository.split_once('/').unwrap();
        let issue = self
            .client
            .issues(owner, repo)
            .create(title)
            .body(body)
            .labels(Some(labels.into_iter().collect()))
            .send()
            .await
            .unwrap();

        issue.id
    }
}
