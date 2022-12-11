use crate::Api;
use app_shared::{
    models::{BugReport, FeatureVote, FeatureVoteDescriptor},
    prelude::*,
};

impl Api {
    #[instrument]
    pub async fn new_feature_vote(&self, vote: FeatureVote) {
        trace!("new_feature_vote api");

        self.database.add_feature_vote(vote).await;
    }

    #[instrument]
    pub async fn end_feature_vote(&self, descriptor: FeatureVoteDescriptor) {
        trace!("end_feature_vote api");

        self.database.end_feature_vote(descriptor).await;
    }

    #[instrument]
    pub async fn is_vote_ended(&self, descriptor: FeatureVoteDescriptor) -> bool {
        trace!("is_vote_ended api");

        self.database.is_vote_ended(descriptor).await
    }

    #[instrument]
    pub async fn get_feature_vote(&self, descriptor: FeatureVoteDescriptor) -> Option<FeatureVote> {
        trace!("get_feature_vote api");

        self.database.get_feature_vote(descriptor).await
    }

    #[instrument]
    pub async fn add_bug_report(&self, bug_report: BugReport) {
        trace!("add_bug_report");

        self.database.add_bug_report(bug_report).await;
    }

    #[instrument]
    pub async fn create_feature_issue(&self, title: String, description: String) -> i64 {
        trace!("create_feature_issue");

        self.github.create_feature_issue(title, description).await
    }

    #[instrument]
    pub async fn create_bug_issue(&self, title: String, description: String) -> i64 {
        trace!("create_bug_issue");

        self.github.create_bug_issue(title, description).await
    }
}
