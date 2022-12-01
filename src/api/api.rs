use crate::api::models::{Account, AnyUserId, NewAccount};
use chrono::{Duration, Utc};
use octocrab::models::IssueId;

use super::{
    github::Github,
    models::{BugReport, FeatureVote, FeatureVoteDescriptor, TFAToken, TokenSecret},
    tfa_tokens_storage::TFATokensStorage,
    Database,
};
use crate::prelude::*;

#[derive(Debug)]
pub struct Api {
    database: Database,
    github: Github,
    tokens_storage: TFATokensStorage,
}

impl Api {
    #[instrument]
    pub async fn new(settings: &Settings) -> Self {
        info!("creating api");

        // GitHub
        let github = Github::new(settings.github.token.clone());

        // Database
        let database = Database::connect(&settings.database.connect).await;
        database.migrate().await;

        // Tokens storage
        let tokens_storage = TFATokensStorage::default();

        Self {
            database,
            github,
            tokens_storage,
        }
    }

    #[instrument]
    pub async fn new_feature_vote(&self, vote: FeatureVote) {
        debug!("new_feature_vote api");

        self.database.add_feature_vote(vote).await;
    }

    #[instrument]
    pub async fn end_feature_vote(&self, descriptor: FeatureVoteDescriptor) {
        debug!("end_feature_vote api");

        self.database.end_feature_vote(descriptor).await;
    }

    #[instrument]
    pub async fn is_vote_ended(&self, descriptor: FeatureVoteDescriptor) -> bool {
        debug!("is_vote_ended api");

        self.database.is_vote_ended(descriptor).await
    }

    #[instrument]
    pub async fn get_feature_vote(&self, descriptor: FeatureVoteDescriptor) -> Option<FeatureVote> {
        debug!("get_feature_vote api");

        self.database.get_feature_vote(descriptor).await
    }

    #[instrument]
    pub async fn add_bug_report(&self, bug_report: BugReport) {
        debug!("add_bug_report");

        self.database.add_bug_report(bug_report).await;
    }

    #[instrument]
    pub async fn create_feature_issue(&self, title: String, description: String) -> IssueId {
        info!("create_feature_issue");

        let settings = Settings::clone_state().await;

        self.github
            .create_issue(
                settings.commands.feedback.features_repository,
                title,
                description,
                settings.commands.feedback.feature_issue_labels,
            )
            .await
    }

    #[instrument]
    pub async fn create_bug_issue(&self, title: String, description: String) -> IssueId {
        info!("create_bug_issue");

        let settings = Settings::clone_state().await;

        self.github
            .create_issue(
                settings.commands.feedback.bugs_repository,
                title,
                description,
                settings.commands.feedback.bug_issue_labels,
            )
            .await
    }

    #[instrument]
    pub async fn get_or_create_tfa_token(&mut self, user: discord::user::User) -> TFAToken {
        debug!("get_or_create_tfa_token");

        if self
            .database
            .find_account(AnyUserId::DiscordId(user.id))
            .await
            .is_none()
        {
            self.database
                .add_account(NewAccount {
                    created_at: Utc::now(),
                    discord_id: user.id,
                })
                .await;
        }

        self.tokens_storage.remove_expired_tokens();

        match self.tokens_storage.find_by_user_id(user.id) {
            None => {
                debug!("existing token not found");

                self.tokens_storage.new_token(user, Duration::seconds(60))
            }
            Some(token) => {
                debug!("existing token found");
                token.clone()
            }
        }
    }

    #[instrument]
    pub async fn find_token_by_secret(&self, secret: TokenSecret) -> Option<TFAToken> {
        debug!("find_token_by_secret");

        self.tokens_storage.find_by_secret(secret).cloned()
    }

    #[instrument]
    pub async fn find_account_by_token_secret(&self, secret: TokenSecret) -> Option<Account> {
        debug!("find_account_by_token_secret");

        let token = self.tokens_storage.find_by_secret(secret);

        let Some(token) = token else {
            return None;
        };

        self.find_account_by_id(AnyUserId::DiscordId(token.user.id))
            .await
    }

    #[instrument]
    pub async fn find_account_by_id(&self, user_id: AnyUserId) -> Option<Account> {
        debug!("find_account_by_id");

        self.database.find_account(user_id).await
    }

    #[instrument]
    pub async fn connect_byond_account(&self, user_id: AnyUserId, byond_user_id: byond::UserId) {
        info!("connect_byond_account");

        let account = self.find_account_by_id(user_id.clone()).await;

        let Some(account) = account else {
            warn!("account not found");
            return;
        };

        if account.byond_ckey.is_some() {
            warn!("byond account is already connected");
            return;
        }

        self.database
            .connect_account(user_id, AnyUserId::ByondCkey(byond_user_id))
            .await;
    }
}
