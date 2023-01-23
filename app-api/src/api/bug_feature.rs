use crate::Api;
use app_shared::{
    models::{BugReport, FeatureVote, FeatureVoteDescriptor},
    prelude::*,
    Database,
};

impl Api {
    /// Сохраняет запись об голосовании.
    #[instrument]
    pub fn new_feature_vote(&self, vote: FeatureVote) {
        trace!("new_feature_vote api");

        Database::lock(|database| database.add_feature_vote(vote));
    }

    /// Отмечает голосование оконченным.
    #[instrument]
    pub fn end_feature_vote(&self, descriptor: FeatureVoteDescriptor) {
        trace!("end_feature_vote api");

        Database::lock(|database| database.end_feature_vote(descriptor));
    }

    /// Проверяет, является ли голосование оконченным.
    #[instrument]
    pub fn is_vote_ended(&self, descriptor: FeatureVoteDescriptor) -> bool {
        trace!("is_vote_ended api");

        Database::lock(|database| database.is_vote_ended(descriptor))
    }

    /// Возвращает информацию о голосовании.
    #[instrument]
    pub fn get_feature_vote(&self, descriptor: FeatureVoteDescriptor) -> Option<FeatureVote> {
        trace!("get_feature_vote api");

        Database::lock(|database| database.get_feature_vote(descriptor))
    }

    /// Сохраняет запись об баг репорте.
    #[instrument]
    pub fn add_bug_report(&self, bug_report: BugReport) {
        trace!("add_bug_report");

        Database::lock(|database| database.add_bug_report(bug_report));
    }

    /// Создаёт иссуй с предложением улучшения на Github.
    #[instrument]
    pub fn create_feature_issue(&self, title: String, description: String) -> i64 {
        trace!("create_feature_issue");

        self.github.create_feature_issue(title, description)
    }

    /// Создаёт иссуй с багом на Github.
    #[instrument]
    pub fn create_bug_issue(&self, title: String, description: String) -> i64 {
        trace!("create_bug_issue");

        self.github.create_bug_issue(title, description)
    }
}
