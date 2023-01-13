use app_macros::config;
use std::collections::BTreeSet;

use app_shared::tokio::runtime::Runtime;
use app_shared::{octocrab::Octocrab, prelude::*};

#[derive(Debug)]
pub struct Github {
    client: Octocrab,
    rt: Runtime,
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
    #[instrument(skip(self))]
    pub fn create_feature_issue(&self, title: String, body: String) -> i64 {
        trace!("create_feature_issue");

        let config = GithubConfig::get().unwrap();

        self.create_issue(
            config.features_repository,
            title,
            body,
            config.feature_issue_labels,
        )
    }

    #[instrument(skip(self))]
    pub fn create_bug_issue(&self, title: String, body: String) -> i64 {
        trace!("create_bug_issue");

        let config = GithubConfig::get().unwrap();

        self.create_issue(config.bugs_repository, title, body, config.bug_issue_labels)
    }

    #[instrument(skip(self))]
    pub fn create_issue(
        &self,
        repository: String,
        title: String,
        body: String,
        labels: BTreeSet<String>,
    ) -> i64 {
        trace!("create_issue");

        let (owner, repo) = repository.split_once('/').unwrap();

        self.rt.block_on(async {
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
        })
    }
}

impl Default for Github {
    fn default() -> Self {
        let rt = app_shared::tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        let config = GithubConfig::get().unwrap();
        let client = Octocrab::builder()
            .personal_token(config.token)
            .build()
            .unwrap();

        Self { client, rt }
    }
}
