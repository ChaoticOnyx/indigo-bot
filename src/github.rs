use std::collections::HashSet;

use crate::prelude::*;
use octocrab::Octocrab;
use once_cell::sync::Lazy;
use serenity::{async_trait, prelude::Mutex};

static GITHUB: Lazy<Mutex<Option<Github>>> = Lazy::new(|| Mutex::new(None));

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
    ) -> i64 {
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

        issue.number
    }
}

#[async_trait]
impl GlobalState for Github {
    #[instrument]
    async fn get_state() -> Github {
        let lock = GITHUB.lock().await;

        lock.clone().unwrap()
    }

    #[instrument]
    async fn set_state(github: Github) {
        let mut lock = GITHUB.lock().await;

        *lock = Some(github);
    }
}
