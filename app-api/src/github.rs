use app_macros::config;
use std::collections::BTreeSet;

use app_shared::{octocrab::Octocrab, prelude::*};

#[derive(Debug, Clone)]
pub struct Github {
    client: Octocrab,
}

#[config]
#[derive(Debug)]
pub struct GithubConfig {
    pub token: String,
    pub features_repository: String,
    pub bugs_repository: String,
    pub bug_issue_labels: BTreeSet<String>,
    pub feature_issue_labels: BTreeSet<String>,
}

impl Github {
    pub async fn new() -> Self {
        let config = GithubConfig::get().await.unwrap();
        let client = Octocrab::builder()
            .personal_token(config.token)
            .build()
            .unwrap();

        Self { client }
    }

    #[instrument(skip(self))]
    pub async fn create_feature_issue(&self, title: String, body: String) -> i64 {
        trace!("create_feature_issue");

        let config = GithubConfig::get().await.unwrap();

        self.create_issue(
            config.features_repository,
            title,
            body,
            config.feature_issue_labels,
        )
        .await
    }

    #[instrument(skip(self))]
    pub async fn create_bug_issue(&self, title: String, body: String) -> i64 {
        trace!("create_bug_issue");

        let config = GithubConfig::get().await.unwrap();
        self.create_issue(config.bugs_repository, title, body, config.bug_issue_labels)
            .await
    }

    #[instrument(skip(self))]
    pub async fn create_issue(
        &self,
        repository: String,
        title: String,
        body: String,
        labels: BTreeSet<String>,
    ) -> i64 {
        trace!("create_issue");

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

        issue.number
    }
}
